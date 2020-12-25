use amethyst_assets::{Asset, Format, ProcessableAsset, ProcessingState};
use amethyst_error::{format_err, Error, ResultExt};
use glyph_brush::rusttype::Font;
use serde::{Deserialize, Serialize};
use type_uuid::TypeUuid;

/// A loaded set of fonts from a file.
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "67bce379-48f7-4a35-bf54-243429a1816b"]
pub struct FontAsset(pub Font<'static>);

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "85bac271-fe10-48da-85d2-151e93ce98d1"]
pub struct FontData(Font<'static>);

amethyst_assets::register_format_type!(FontData);

impl Asset for FontAsset {
    fn name() -> &'static str {
        "ui::Font"
    }
    type Data = FontData;
}

impl ProcessableAsset for FontAsset {
    fn process(data: FontData) -> Result<ProcessingState<FontData, FontAsset>, Error> {
        log::debug!("Loading Font");
        Ok(ProcessingState::Loaded(FontAsset(data.0)))
    }
}

/// Loads font files, supports TrueType and **some** OpenType files.
///
/// OpenType is a superset of TrueType, so if your OpenType file uses any features that don't
/// exist in TrueType this will fail.  This will only load the first font contained in a file.
/// If this is a problem for you please file an issue with Amethyst on GitHub.
#[derive(Clone, Debug, Default, Serialize, Deserialize, TypeUuid)]
#[uuid = "2e974cc5-c0ad-4db5-8d43-40e7c69373d7"]
pub struct TtfFormat;

amethyst_assets::register_format!("TTF", TtfFormat as FontData);
// FontData does not implement Serialize/Deserialize, so we cannot register importer :(
//amethyst_assets::register_importer!(".ttf", TtfFormat);
impl Format<FontData> for TtfFormat {
    fn name(&self) -> &'static str {
        "TTF"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<FontData, Error> {
        Font::from_bytes(bytes)
            .map(FontData)
            .with_context(|_| format_err!("Font parsing error"))
    }
}
