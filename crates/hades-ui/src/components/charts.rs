use std::rc::Rc;

use gpui::{
    App, Bounds, ElementId, Hsla, InteractiveElement, IntoElement, MouseMoveEvent, ParentElement,
    PathBuilder, Pixels, Point, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, canvas, div, linear_color_stop, linear_gradient, point, px, relative,
};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

/// Margens da área de plotagem do `AreaChart` (`m` em `kit/panel/ui/app.tsx:535`), em px.
const MARGIN_LEFT: f32 = 42.0;
const MARGIN_RIGHT: f32 = 14.0;
const MARGIN_TOP: f32 = 14.0;
const MARGIN_BOTTOM: f32 = 26.0;
/// Folga acima do maior valor, para a curva não encostar no topo.
const HEADROOM: f64 = 1.15;
const Y_TICKS: usize = 4;
/// Largura de cada rótulo do eixo x e espaço mínimo entre eles.
const X_LABEL_WIDTH: f32 = 64.0;
const X_LABEL_SPACING: f32 = 110.0;
const TOOLTIP_WIDTH: f32 = 170.0;

/// Passo redondo (1, 2, 2,5, 5 × 10ⁿ) que divide `max` em até `n` partes.
///
/// Porta `niceStep` (`kit/panel/ui/app.tsx:503-506`).
pub fn nice_step(max: f64, n: usize) -> f64 {
    let step0 = max / n.max(1) as f64;
    if step0 <= 0.0 || !step0.is_finite() {
        return 1.0;
    }
    let mag = 10f64.powf(step0.log10().floor());
    [1.0, 2.0, 2.5, 5.0, 10.0]
        .iter()
        .map(|f| f * mag)
        .find(|s| *s >= step0)
        .unwrap_or(mag * 10.0)
}

/// Marcas do eixo y de 0 até `max`, no passo de [`nice_step`].
///
/// Porta `niceTicks` (`kit/panel/ui/app.tsx:507-512`).
pub fn nice_ticks(max: f64, n: usize) -> Vec<f64> {
    let step = nice_step(max, n);
    let mut out = Vec::new();
    let mut i = 0usize;
    loop {
        let v = ((i as f64 * step) * 1000.0).round() / 1000.0;
        if v > max + 1e-9 {
            break;
        }
        out.push(v);
        i += 1;
    }
    out
}

/// Topo da escala y: o maior valor (ou a linha de referência, se maior) com 15 % de folga; nunca zero.
///
/// Porta `ymax` de `AreaChart` (`kit/panel/ui/app.tsx:539`).
pub fn chart_y_max<'a>(values: impl IntoIterator<Item = &'a f64>, ref_line: Option<f64>) -> f64 {
    values
        .into_iter()
        .copied()
        .chain(ref_line)
        .fold(1e-6, f64::max)
        * HEADROOM
}

/// Frase no lugar do gráfico quando não há o que desenhar, ou `None` quando há.
///
/// Porta o estado vazio de `AreaChart` (`kit/panel/ui/app.tsx:551`).
pub fn chart_empty_message(points: usize, has_value: bool) -> Option<&'static str> {
    match points {
        0 => Some("Sem amostras ainda."),
        1 => Some("Coletando amostras…"),
        _ if !has_value => Some("Sem atividade no período."),
        _ => None,
    }
}

/// Posição x do ponto `i` de `n`, distribuídos por igual de `left` a `left + width`.
pub fn x_at(i: usize, n: usize, left: f32, width: f32) -> f32 {
    if n <= 1 {
        return left;
    }
    left + i as f32 / (n - 1) as f32 * width
}

/// Posição y do valor `v` numa escala de 0 a `max` desenhada de `top + height` (zero) até `top`.
pub fn y_at(v: f64, max: f64, top: f32, height: f32) -> f32 {
    let frac = if max > 0.0 {
        (v / max).clamp(0.0, 1.0)
    } else {
        0.0
    };
    top + height - frac as f32 * height
}

/// Ponto mais perto da posição `x`, entre `n` distribuídos de `left` a `left + width`.
///
/// Porta `move` de `AreaChart` (`kit/panel/ui/app.tsx:542-548`).
pub fn nearest_index(x: f32, n: usize, left: f32, width: f32) -> Option<usize> {
    if n == 0 {
        return None;
    }
    if n == 1 || width <= 0.0 {
        return Some(0);
    }
    let frac = ((x - left) / width).clamp(0.0, 1.0);
    Some((frac * (n - 1) as f32).round() as usize)
}

/// Pontos que ganham rótulo no eixo x: de 2 a 6 divisões, conforme a largura, sempre com o primeiro e o último.
///
/// Porta a escolha de rótulos de `AreaChart` (`kit/panel/ui/app.tsx:563`), sobre índices em vez de horários.
pub fn x_label_indices(points: usize, width: f32) -> Vec<usize> {
    if points == 0 {
        return Vec::new();
    }
    if points == 1 {
        return vec![0];
    }
    let divisions = ((width / X_LABEL_SPACING).floor() as usize).clamp(2, 6);
    let mut out: Vec<usize> = (0..=divisions)
        .map(|i| ((i * (points - 1)) as f32 / divisions as f32).round() as usize)
        .collect();
    out.dedup();
    out
}

/// Pontos da linha da `Sparkline` numa caixa de `width` × `height`: a escala vai do menor ao maior valor da
/// série; série constante fica no meio.
pub fn sparkline_points(values: &[f64], width: f32, height: f32) -> Vec<(f32, f32)> {
    let lo = values.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = hi - lo;
    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let y = if span > 0.0 {
                height - ((v - lo) / span) as f32 * height
            } else {
                height / 2.0
            };
            (x_at(i, values.len(), 0.0, width), y)
        })
        .collect()
}

fn to_point(origin: Point<Pixels>, (x, y): (f32, f32)) -> Point<Pixels> {
    point(origin.x + px(x), origin.y + px(y))
}

fn paint_line(window: &mut Window, pts: &[Point<Pixels>], width: f32, color: Hsla) {
    if pts.len() < 2 {
        return;
    }
    let mut path = PathBuilder::stroke(px(width));
    path.move_to(pts[0]);
    for p in &pts[1..] {
        path.line_to(*p);
    }
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}

/// Linha tracejada: traço de `dash` e vão de `gap` px ao longo de cada segmento.
fn paint_dashed(
    window: &mut Window,
    pts: &[Point<Pixels>],
    width: f32,
    color: Hsla,
    dash: f32,
    gap: f32,
) {
    for seg in pts.windows(2) {
        let (a, b) = (seg[0], seg[1]);
        let (dx, dy) = (f32::from(b.x - a.x), f32::from(b.y - a.y));
        let len = (dx * dx + dy * dy).sqrt();
        if len <= 0.0 {
            continue;
        }
        let mut t = 0.0;
        while t < len {
            let end = (t + dash).min(len);
            let at = |d: f32| point(a.x + px(dx * d / len), a.y + px(dy * d / len));
            paint_line(window, &[at(t), at(end)], width, color);
            t = end + gap;
        }
    }
}

/// Área sob a linha até `base_y`, em degradê da cor (28 % no topo, transparente na base).
fn paint_area(window: &mut Window, pts: &[Point<Pixels>], base_y: Pixels, color: Hsla) {
    if pts.len() < 2 {
        return;
    }
    let mut path = PathBuilder::fill();
    path.move_to(point(pts[0].x, base_y));
    for p in pts {
        path.line_to(*p);
    }
    path.line_to(point(pts[pts.len() - 1].x, base_y));
    path.close();
    if let Ok(path) = path.build() {
        window.paint_path(
            path,
            linear_gradient(
                180.0,
                linear_color_stop(color.opacity(0.28), 0.0),
                linear_color_stop(color.opacity(0.0), 1.0),
            ),
        );
    }
}

/// Tendência curta (a última hora de um serviço, de um gate) sem eixos: linha de 1,5 px com área em degradê e o
/// último ponto marcado. A escala vai do menor ao maior valor; os dados vêm todos do chamador.
#[derive(IntoElement)]
pub struct Sparkline {
    values: Vec<f64>,
    color: Option<Hsla>,
    width: f32,
    height: f32,
}

impl Sparkline {
    pub fn new(values: impl IntoIterator<Item = f64>) -> Self {
        Self {
            values: values.into_iter().collect(),
            color: None,
            width: 120.0,
            height: 28.0,
        }
    }

    /// Cor da linha; por padrão a `primary`.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl RenderOnce for Sparkline {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let color = self.color.unwrap_or(p.primary);
        let card = p.card;
        let (w, h) = (self.width, self.height);
        let values = self.values;
        div().flex_none().w(px(w)).h(px(h)).child(
            canvas(
                |_, _, _| (),
                move |b: Bounds<Pixels>, _, window, _| {
                    let inset = 3.0;
                    let pts: Vec<Point<Pixels>> =
                        sparkline_points(&values, w - 2.0 * inset, h - 2.0 * inset)
                            .into_iter()
                            .map(|(x, y)| to_point(b.origin, (x + inset, y + inset)))
                            .collect();
                    paint_area(window, &pts, b.origin.y + px(h), color);
                    paint_line(window, &pts, 1.5, color);
                    if let Some(last) = pts.last() {
                        let r = px(2.5);
                        window.paint_quad(
                            gpui::fill(
                                Bounds::new(
                                    point(last.x - r, last.y - r),
                                    gpui::size(r * 2.0, r * 2.0),
                                ),
                                color,
                            )
                            .corner_radii(r)
                            .border_widths(px(1.0))
                            .border_color(card),
                        );
                    }
                },
            )
            .size_full(),
        )
    }
}

/// Uma série do [`AreaChart`]: nome na dica, um valor por ponto, cor, tracejada ou sem área.
#[derive(Clone, Debug)]
pub struct ChartSeries {
    pub name: SharedString,
    pub values: Vec<f64>,
    pub color: Hsla,
    pub dashed: bool,
    pub fill: bool,
}

impl ChartSeries {
    pub fn new(
        name: impl Into<SharedString>,
        values: impl IntoIterator<Item = f64>,
        color: Hsla,
    ) -> Self {
        Self {
            name: name.into(),
            values: values.into_iter().collect(),
            color,
            dashed: false,
            fill: true,
        }
    }

    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill = false;
        self
    }
}

type Formatter = Rc<dyn Fn(f64) -> String>;

#[derive(Default)]
struct HoverState {
    bounds: Option<Bounds<Pixels>>,
    index: Option<usize>,
}

/// Gráfico de área (memória na última hora, consumo por dia): pontos igualmente espaçados com um rótulo cada
/// (hora, dia), uma ou mais séries, grade tracejada com 4 marcas redondas no eixo y, linha de referência opcional
/// (teto) e dica com os valores do ponto sob o mouse. Os dados vêm todos do chamador.
///
/// Porta `AreaChart` (`kit/panel/ui/app.tsx:531-600`).
#[derive(IntoElement)]
pub struct AreaChart {
    id: ElementId,
    labels: Vec<SharedString>,
    series: Vec<ChartSeries>,
    height: f32,
    y_fmt: Formatter,
    ref_line: Option<(f64, SharedString)>,
}

impl AreaChart {
    pub fn new(id: impl Into<ElementId>, labels: impl IntoIterator<Item = SharedString>) -> Self {
        Self {
            id: id.into(),
            labels: labels.into_iter().collect(),
            series: Vec::new(),
            height: 220.0,
            y_fmt: Rc::new(|v| format!("{v}")),
            ref_line: None,
        }
    }

    pub fn series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Formato dos valores no eixo y e na dica.
    pub fn y_fmt(mut self, f: impl Fn(f64) -> String + 'static) -> Self {
        self.y_fmt = Rc::new(f);
        self
    }

    /// Linha tracejada em `bad` no valor `value`, com o rótulo à direita (ex.: "teto de memória").
    pub fn ref_line(mut self, value: f64, label: impl Into<SharedString>) -> Self {
        self.ref_line = Some((value, label.into()));
        self
    }
}

impl RenderOnce for AreaChart {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let n = self.labels.len();
        let has_value = self
            .series
            .iter()
            .any(|s| s.values.iter().take(n).any(|v| *v != 0.0));
        if let Some(msg) = chart_empty_message(n, has_value) {
            return div()
                .py(px(8.0))
                .text_size(px(size::BODY))
                .text_color(p.muted_fg)
                .child(msg)
                .into_any_element();
        }

        let hover =
            window.use_keyed_state((self.id.clone(), "mouse"), cx, |_, _| HoverState::default());
        let height = self.height;
        let plot_h = height - MARGIN_TOP - MARGIN_BOTTOM;
        let ymax = chart_y_max(
            self.series.iter().flat_map(|s| s.values.iter().take(n)),
            self.ref_line.as_ref().map(|r| r.0),
        );
        let ticks = nice_ticks(ymax, Y_TICKS);
        let known_width = hover
            .read(cx)
            .bounds
            .map(|b| f32::from(b.size.width))
            .unwrap_or(600.0);
        let plot_w = (known_width - MARGIN_LEFT - MARGIN_RIGHT).max(1.0);
        let hovered = hover.read(cx).index.filter(|i| *i < n);

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .min_w_0()
            .h(px(height))
            .text_size(px(size::MICRO))
            .text_color(p.muted_fg);

        root = root.children(ticks.iter().map(|v| {
            div()
                .absolute()
                .left_0()
                .w(px(MARGIN_LEFT - 8.0))
                .top(px(y_at(*v, ymax, MARGIN_TOP, plot_h) - 7.0))
                .flex()
                .justify_end()
                .font_family(FONT_MONO)
                .child((self.y_fmt)(*v))
        }));

        let last = n.saturating_sub(1).max(1) as f32;
        let x_labels = div()
            .absolute()
            .left(px(MARGIN_LEFT))
            .right(px(MARGIN_RIGHT))
            .bottom(px(4.0))
            .h(px(14.0))
            .children(x_label_indices(n, known_width).into_iter().map(|i| {
                let frac = i as f32 / last;
                let mut label = div()
                    .absolute()
                    .top_0()
                    .w(px(X_LABEL_WIDTH))
                    .flex()
                    .child(self.labels[i].clone());
                label = if i == 0 {
                    label.left_0()
                } else if i == n - 1 {
                    label.right_0().justify_end()
                } else {
                    label
                        .left(relative(frac))
                        .ml(px(-X_LABEL_WIDTH / 2.0))
                        .justify_center()
                };
                label
            }));
        root = root.child(x_labels);

        if let Some((value, label)) = self.ref_line.clone() {
            root = root.child(
                div()
                    .absolute()
                    .right(px(MARGIN_RIGHT))
                    .top(px(y_at(value, ymax, MARGIN_TOP, plot_h) - 18.0))
                    .text_color(p.bad)
                    .child(label),
            );
        }

        let series = self.series.clone();
        let ref_value = self.ref_line.as_ref().map(|r| r.0);
        let (border, bad, ring, card) = (p.border, p.bad, p.fg.opacity(0.5), p.card);
        let hover_paint = hover.clone();
        root = root.child(
            canvas(
                move |b, _, cx| {
                    hover_paint.update(cx, |h, _| h.bounds = Some(b));
                },
                move |b: Bounds<Pixels>, _, window, _| {
                    let w = f32::from(b.size.width);
                    let pw = (w - MARGIN_LEFT - MARGIN_RIGHT).max(1.0);
                    let at = |x: f32, y: f32| to_point(b.origin, (x, y));
                    for v in nice_ticks(ymax, Y_TICKS) {
                        let y = y_at(v, ymax, MARGIN_TOP, plot_h);
                        let line = [at(MARGIN_LEFT, y), at(w - MARGIN_RIGHT, y)];
                        if v == 0.0 {
                            paint_line(window, &line, 1.0, border);
                        } else {
                            paint_dashed(window, &line, 1.0, border, 3.0, 3.0);
                        }
                    }
                    if let Some(v) = ref_value {
                        let y = y_at(v, ymax, MARGIN_TOP, plot_h);
                        paint_dashed(
                            window,
                            &[at(MARGIN_LEFT, y), at(w - MARGIN_RIGHT, y)],
                            1.5,
                            bad,
                            4.0,
                            4.0,
                        );
                    }
                    let base = b.origin.y + px(y_at(0.0, ymax, MARGIN_TOP, plot_h));
                    for s in &series {
                        let pts: Vec<Point<Pixels>> = s
                            .values
                            .iter()
                            .take(n)
                            .enumerate()
                            .map(|(i, v)| {
                                at(
                                    x_at(i, n, MARGIN_LEFT, pw),
                                    y_at(*v, ymax, MARGIN_TOP, plot_h),
                                )
                            })
                            .collect();
                        if s.fill {
                            paint_area(window, &pts, base, s.color);
                        }
                        if s.dashed {
                            paint_dashed(window, &pts, 2.0, s.color, 5.0, 4.0);
                        } else {
                            paint_line(window, &pts, 2.0, s.color);
                        }
                    }
                    if let Some(i) = hovered {
                        let x = x_at(i, n, MARGIN_LEFT, pw);
                        paint_line(
                            window,
                            &[at(x, MARGIN_TOP), at(x, height - MARGIN_BOTTOM)],
                            1.0,
                            ring,
                        );
                        for s in &series {
                            let Some(v) = s.values.get(i) else { continue };
                            let c = at(x, y_at(*v, ymax, MARGIN_TOP, plot_h));
                            let r = px(4.0);
                            window.paint_quad(
                                gpui::fill(
                                    Bounds::new(
                                        point(c.x - r, c.y - r),
                                        gpui::size(r * 2.0, r * 2.0),
                                    ),
                                    s.color,
                                )
                                .corner_radii(r)
                                .border_widths(px(2.0))
                                .border_color(card),
                            );
                        }
                    }
                },
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        );

        if let Some(i) = hovered {
            let x = x_at(i, n, MARGIN_LEFT, plot_w);
            let left = (x + 12.0).max(0.0).min(known_width - TOOLTIP_WIDTH);
            let mut tip = div()
                .absolute()
                .top(px(4.0))
                .left(px(left))
                .w(px(TOOLTIP_WIDTH))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .px(px(10.0))
                .py(px(6.0))
                .bg(p.popover)
                .shadow(vec![p.dialog_shadow()])
                .text_size(px(size::META))
                .text_color(p.fg)
                .child(self.labels[i].clone());
            for s in &self.series {
                let value = s
                    .values
                    .get(i)
                    .map(|v| (self.y_fmt)(*v))
                    .unwrap_or_default();
                tip = tip.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(div().size(px(10.0)).flex_none().bg(s.color))
                        .child(div().text_color(p.muted_fg).child(s.name.clone()))
                        .child(div().ml_auto().font_family(FONT_MONO).child(value)),
                );
            }
            root = root.child(tip);
        }

        let hover_move = hover.clone();
        let hover_leave = hover;
        root.on_mouse_move(move |e: &MouseMoveEvent, _, cx| {
            hover_move.update(cx, |h, cx| {
                let Some(b) = h.bounds else { return };
                let x = f32::from(e.position.x - b.origin.x);
                let pw = f32::from(b.size.width) - MARGIN_LEFT - MARGIN_RIGHT;
                let inside = b.contains(&e.position);
                let index = inside
                    .then(|| nearest_index(x, n, MARGIN_LEFT, pw))
                    .flatten();
                if index != h.index {
                    h.index = index;
                    cx.notify();
                }
            })
        })
        .on_hover(move |over, _, cx| {
            if !*over {
                hover_leave.update(cx, |h, cx| {
                    h.index = None;
                    cx.notify();
                })
            }
        })
        .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passo_redondo_divide_o_maximo() {
        assert_eq!(nice_step(10.0, 4), 2.5);
        assert_eq!(nice_step(7.0, 4), 2.0);
        assert_eq!(nice_step(0.9, 4), 0.25);
        assert_eq!(nice_step(1150.0, 4), 500.0);
        assert_eq!(nice_step(0.0, 4), 1.0);
    }

    #[test]
    fn marcas_vao_de_zero_ao_maximo_sem_passar() {
        assert_eq!(nice_ticks(10.0, 4), vec![0.0, 2.5, 5.0, 7.5, 10.0]);
        assert_eq!(nice_ticks(9.2, 4), vec![0.0, 2.5, 5.0, 7.5]);
        assert_eq!(nice_ticks(0.3, 4), vec![0.0, 0.1, 0.2, 0.3]);
    }

    #[test]
    fn topo_tem_folga_e_respeita_a_referencia() {
        assert!((chart_y_max(&[1.0, 4.0, 2.0], None) - 4.6).abs() < 1e-9);
        assert!((chart_y_max(&[1.0], Some(10.0)) - 11.5).abs() < 1e-9);
        assert!(chart_y_max(&[0.0, 0.0], None) > 0.0);
    }

    #[test]
    fn estado_vazio_segue_o_painel() {
        assert_eq!(chart_empty_message(0, false), Some("Sem amostras ainda."));
        assert_eq!(chart_empty_message(1, true), Some("Coletando amostras…"));
        assert_eq!(
            chart_empty_message(5, false),
            Some("Sem atividade no período.")
        );
        assert_eq!(chart_empty_message(5, true), None);
    }

    #[test]
    fn escala_leva_zero_a_base_e_o_maximo_ao_topo() {
        assert_eq!(y_at(0.0, 10.0, 14.0, 180.0), 194.0);
        assert_eq!(y_at(10.0, 10.0, 14.0, 180.0), 14.0);
        assert_eq!(y_at(5.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(y_at(20.0, 10.0, 0.0, 100.0), 0.0);
        assert_eq!(x_at(0, 5, 42.0, 400.0), 42.0);
        assert_eq!(x_at(4, 5, 42.0, 400.0), 442.0);
        assert_eq!(x_at(0, 1, 42.0, 400.0), 42.0);
    }

    #[test]
    fn mouse_escolhe_o_ponto_mais_perto() {
        assert_eq!(nearest_index(0.0, 5, 42.0, 400.0), Some(0));
        assert_eq!(nearest_index(150.0, 5, 42.0, 400.0), Some(1));
        assert_eq!(nearest_index(1000.0, 5, 42.0, 400.0), Some(4));
        assert_eq!(nearest_index(10.0, 0, 0.0, 100.0), None);
        assert_eq!(nearest_index(80.0, 1, 0.0, 100.0), Some(0));
    }

    #[test]
    fn rotulos_do_eixo_x_cobrem_as_pontas() {
        assert_eq!(x_label_indices(61, 660.0), vec![0, 10, 20, 30, 40, 50, 60]);
        assert_eq!(x_label_indices(61, 200.0), vec![0, 30, 60]);
        assert_eq!(x_label_indices(3, 1000.0), vec![0, 1, 2]);
        assert_eq!(x_label_indices(1, 500.0), vec![0]);
        assert!(x_label_indices(0, 500.0).is_empty());
    }

    #[test]
    fn sparkline_vai_do_menor_ao_maior() {
        let pts = sparkline_points(&[2.0, 4.0, 3.0], 100.0, 20.0);
        assert_eq!(pts, vec![(0.0, 20.0), (50.0, 0.0), (100.0, 10.0)]);
        let flat = sparkline_points(&[5.0, 5.0], 10.0, 20.0);
        assert_eq!(flat, vec![(0.0, 10.0), (10.0, 10.0)]);
    }
}
