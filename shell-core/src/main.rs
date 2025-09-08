use anyhow::Result;
use qmetaobject::prelude::*;

#[derive(QObject, Default)]
struct ThemeModel {
    base: qt_base_class!(trait QObject),
    #[qt_property(QString)]
    color_brand_orange: QString,
}

fn main() -> Result<()> {
    qmetaobject::log::init_qt_to_rust();
    let engine = QmlEngine::new();
    let mut theme = ThemeModel::default();
    theme.color_brand_orange = QString::from("#ff3e00");
    engine.set_object_property("themeModel".into(), theme.into_qobject());
    engine.load_data(r#"
        import QtQuick 2.15
        import QtQuick.Window 2.15
        Window {
          width: 900; height: 36; visible: true; color: "transparent"
          Rectangle {
            anchors.fill: parent
            color: themeModel.color_brand_orange
          }
        }
    "#.into());
    engine.exec();
    Ok(())
}
