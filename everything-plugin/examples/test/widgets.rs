//! https://github.com/compio-rs/winio/blob/ca97049907a0151168100365ce5e13410f508792/winio/examples/widgets.rs
//!
//! Legacy style: https://github.com/Chaoses-Ib/IbEverythingLib/blob/c8e6e5f175cff2b4ff2e93acf4c265e9c59ddb39/everything-plugin/examples/test/widgets.rs
#![allow(unused_must_use)]

use everything_plugin::ui::winio::prelude::*;

use crate::{App, HANDLER};

#[allow(dead_code)]
pub fn main() {
    // #[cfg(feature = "enable_log")]
    // tracing_subscriber::fmt()
    //     .with_max_level(compio_log::Level::INFO)
    //     .init();

    winio::ui::App::new("rs.compio.winio.widgets")
        .unwrap()
        .run::<MainModel>(())
        .unwrap();
}

pub struct MainModel {
    window: Child<View>,
    ulabel: Child<Label>,
    plabel: Child<Label>,
    uentry: Child<Edit>,
    pentry: Child<Edit>,
    pcheck: Child<CheckBox>,
    canvas: Child<Canvas>,
    combo: Child<ComboBox>,
    list: Child<ObservableVec<String>>,
    index: Option<usize>,
    radio_group: Child<RadioButtonGroup>,
    rindex: usize,
    push_button: Child<Button>,
    pop_button: Child<Button>,
    show_button: Child<Button>,
    progress: Child<Progress>,
    mltext: Child<TextBox>,
}

#[derive(Debug)]
pub enum MainMessage {
    Noop,
    List(ObservableVecEvent<String>),
    Select,
    Push,
    Pop,
    Show,
    RSelect(usize),
    PasswordCheck,
    OptionsPage(OptionsPageMessage<App>),
}

impl From<OptionsPageMessage<App>> for MainMessage {
    fn from(value: OptionsPageMessage<App>) -> Self {
        Self::OptionsPage(value)
    }
}

impl Component for MainModel {
    type Event = ();
    type Init<'a> = OptionsPageInit<'a, App>;
    type Message = MainMessage;
    type Error = Error;

    async fn init(
        mut init: Self::Init<'_>,
        sender: &ComponentSender<Self>,
    ) -> Result<Self, Self::Error> {
        // let mut window = Child::<Window>::init(init);
        let mut window = init.window(sender).await?;
        // window.set_text("Widgets example");
        window.set_size(Size::new(800.0, 600.0));

        let canvas = Child::<Canvas>::init(&window).await?;
        let mut ulabel = Child::<Label>::init(&window).await?;
        ulabel.set_text("Username:");
        ulabel.set_halign(HAlign::Right);
        let mut plabel = Child::<Label>::init(&window).await?;
        plabel.set_text("Password:");
        plabel.set_halign(HAlign::Right);
        let mut uentry = Child::<Edit>::init(&window).await?;
        uentry.set_text("AAA");
        let mut pentry = Child::<Edit>::init(&window).await?;
        pentry.set_text("123456");
        pentry.set_password(true);
        let mut pcheck = Child::<CheckBox>::init(&window).await?;
        pcheck.set_text("Show");
        pcheck.set_checked(false);
        let combo = Child::<ComboBox>::init(&window).await?;
        let mut list = Child::<ObservableVec<String>>::init(Vec::new())
            .await
            .unwrap();
        // https://www.zhihu.com/question/23600507/answer/140640887
        list.push("烫烫烫".to_string());
        list.push("昍昍昍".to_string());
        list.push("ﾌﾌﾌﾌﾌﾌ".to_string());
        list.push("쳌쳌쳌".to_string());
        let mut r1 = Child::<RadioButton>::init(&window).await?;
        r1.set_text("屯屯屯");
        r1.set_checked(true);
        let mut r2 = Child::<RadioButton>::init(&window).await?;
        r2.set_text("锟斤拷");
        let mut r3 = Child::<RadioButton>::init(&window).await?;
        r3.set_text("╠╠╠");
        // Initialize radio group with the radio buttons
        let radio_group = Child::<RadioButtonGroup>::init(vec![r1, r2, r3]).await?;

        let mut push_button = Child::<Button>::init(&window).await?;
        push_button.set_text("Push");
        let mut pop_button = Child::<Button>::init(&window).await?;
        pop_button.set_text("Pop");
        let mut show_button = Child::<Button>::init(&window).await?;
        show_button.set_text("Show");
        let mut progress = Child::<Progress>::init(&window).await?;
        progress.set_indeterminate(true);
        let mut mltext = Child::<TextBox>::init(&window).await?;
        HANDLER.with_app(|a| mltext.set_text(&a.config().s));

        window.show();

        Ok(Self {
            window,
            ulabel,
            plabel,
            uentry,
            pentry,
            pcheck,
            canvas,
            combo,
            list,
            index: None,
            radio_group,
            rindex: 0,
            push_button,
            pop_button,
            show_button,
            progress,
            mltext,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        start! {
            sender, default: MainMessage::Noop,
            self.pcheck => {
                CheckBoxEvent::Click => MainMessage::PasswordCheck,
            },
            self.combo => {
                ComboBoxEvent::Select => MainMessage::Select,
            },
            self.push_button => {
                ButtonEvent::Click => MainMessage::Push,
            },
            self.pop_button => {
                ButtonEvent::Click => MainMessage::Pop,
            },
            self.show_button => {
                ButtonEvent::Click => MainMessage::Show,
            },
            self.list => {
                e => MainMessage::List(e),
            },
            self.radio_group => {
                RadioButtonGroupEvent::Click(i) => MainMessage::RSelect(i)
            }
        }
    }

    async fn update_children(&mut self) -> Result<bool, Self::Error> {
        update_children!(self.window, self.canvas, self.radio_group)
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool, Self::Error> {
        // futures_util::future::join(self.window.update(), self.canvas.update()).await;
        Ok(match message {
            MainMessage::Noop => false,
            MainMessage::PasswordCheck => {
                self.pentry.set_password(!self.pcheck.is_checked()?);
                true
            }
            MainMessage::List(e) => {
                self.pop_button.set_enabled(!self.list.is_empty());
                self.combo
                    .emit(ComboBoxMessage::from_observable_vec_event(e))
                    .await?
            }
            MainMessage::Select => {
                self.index = self.combo.selection()?;
                false
            }
            MainMessage::Push => {
                self.list.push(self.radio_group[self.rindex].text()?);
                false
            }
            MainMessage::Pop => {
                self.list.pop();
                false
            }
            MainMessage::RSelect(i) => {
                self.rindex = i;
                false
            }
            MainMessage::Show => {
                MessageBox::new()
                    .title("Show selected item")
                    .message(
                        self.index
                            .and_then(|index| self.list.get(index))
                            .map(|s| s.as_str())
                            .unwrap_or("No selection."),
                    )
                    .buttons(MessageBoxButton::Ok)
                    // https://github.com/compio-rs/winio/issues/105
                    .show(unsafe { BorrowedWindow::win32(self.window.as_container().as_win32()) })
                    .await;
                false
            }
            MainMessage::OptionsPage(m) => {
                tracing::debug!(?m, "Options page message");
                match m {
                    OptionsPageMessage::Redraw => true,
                    OptionsPageMessage::Close => {
                        sender.output(());
                        false
                    }
                    OptionsPageMessage::Save(config, tx) => {
                        config.s = self.mltext.text()?;
                        tx.send(config).unwrap();
                        false
                    }
                }
            }
        })
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<(), Self::Error> {
        let csize = self.window.size()?;
        {
            let mut cred_panel = layout! {
                Grid::from_str("auto,1*,auto", "1*,auto,auto,1*").unwrap(),
                self.ulabel => { column: 0, row: 1, valign: VAlign::Center },
                self.uentry => { column: 1, row: 1, margin: Margin::new_all_same(4.0) },
                self.plabel => { column: 0, row: 2, valign: VAlign::Center },
                self.pentry => { column: 1, row: 2, margin: Margin::new_all_same(4.0) },
                self.pcheck => { column: 2, row: 2 },
            };

            let mut rgroup_panel = Grid::from_str("auto", "1*,auto,auto,auto,1*").unwrap();
            for (i, rb) in self.radio_group.iter_mut().enumerate() {
                rgroup_panel.push(rb).row(i + 1).finish();
            }

            let mut buttons_panel = layout! {
                StackPanel::new(Orient::Vertical),
                self.push_button => { margin: Margin::new_all_same(4.0) },
                self.pop_button  => { margin: Margin::new_all_same(4.0) },
                self.show_button => { margin: Margin::new_all_same(4.0) },
            };

            let mut root_panel = layout! {
                Grid::from_str("1*,1*,1*", "1*,auto,1*").unwrap(),
                cred_panel    => { column: 1, row: 0 },
                rgroup_panel  => { column: 2, row: 0, halign: HAlign::Center },
                self.canvas   => { column: 0, row: 1, row_span: 2 },
                self.combo    => { column: 1, row: 1, halign: HAlign::Center },
                self.progress => { column: 2, row: 1 },
                self.mltext   => { column: 1, row: 2, margin: Margin::new_all_same(8.0) },
                buttons_panel => { column: 2, row: 2 },
            };

            root_panel.set_size(csize);
        }

        let size = self.canvas.size()?;
        let is_dark = ColorTheme::current()? == ColorTheme::Dark;
        let back_color = if is_dark {
            Color::new(255, 255, 255, 255)
        } else {
            Color::new(0, 0, 0, 255)
        };
        let brush = SolidColorBrush::new(back_color);
        let pen = BrushPen::new(&brush, 1.0);
        let mut ctx = self.canvas.context()?;
        let cx = size.width / 2.0;
        let cy = size.height / 2.0;
        let r = cx.min(cy) - 2.0;
        ctx.draw_pie(
            &pen,
            Rect::new(Point::new(cx - r, cy - r), Size::new(r * 2.0, r * 2.0)),
            std::f64::consts::PI,
            std::f64::consts::PI * 2.0,
        );

        let brush2 = LinearGradientBrush::new(
            [
                GradientStop::new(Color::new(0x87, 0xCE, 0xEB, 0xFF), 0.0),
                GradientStop::new(back_color, 1.0),
            ],
            RelativePoint::zero(),
            RelativePoint::new(0.0, 1.0),
        );
        let pen2 = BrushPen::new(&brush2, 1.0);
        ctx.draw_round_rect(
            &pen2,
            Rect::new(
                Point::new(cx - r - 1.0, cy - r - 1.0),
                Size::new(r * 2.0 + 2.0, r * 1.618 + 2.0),
            ),
            Size::new(r / 10.0, r / 10.0),
        );
        let mut path = ctx.create_path_builder(Point::new(cx + r + 1.0 - r / 10.0, cy))?;
        path.add_arc(
            Point::new(cx, cy + r * 0.618 + 1.0),
            Size::new(r + 1.0 - r / 10.0, r * 0.382 / 2.0),
            0.0,
            std::f64::consts::PI,
            true,
        );
        path.add_line(Point::new(cx - r - 1.0 + r / 10.0, cy));
        let path = path.build(false)?;
        ctx.draw_path(&pen, &path);
        let brush3 = RadialGradientBrush::new(
            [
                GradientStop::new(Color::new(0xF5, 0xF5, 0xF5, 0xFF), 0.0),
                GradientStop::new(
                    Color::accent().unwrap_or(Color::new(0xFF, 0xC0, 0xCB, 0xFF)),
                    1.0,
                ),
            ],
            RelativePoint::new(0.5, 0.5),
            RelativePoint::new(0.2, 0.5),
            RelativeSize::new(0.5, 0.5),
        );
        let font = DrawingFontBuilder::new()
            .family("Arial")
            .size(r / 5.0)
            .halign(HAlign::Center)
            .valign(VAlign::Bottom)
            .build();
        ctx.draw_str(&brush3, font, Point::new(cx, cy), "Hello world!");
        Ok(())
    }
}
