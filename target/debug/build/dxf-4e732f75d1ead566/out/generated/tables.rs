// The contents of this file are automatically generated and should not be modified directly.  See the `build` directory.

use crate::{
    CodePair,
    CodePairValue,
    Color,
    Drawing,
    DrawingItem,
    DrawingItemMut,
    DxfError,
    DxfResult,
    ExtensionGroup,
    Handle,
    LineWeight,
    Point,
    Vector,
    XData,
};
use crate::code_pair_put_back::CodePairPutBack;
use crate::helper_functions::*;
use crate::extension_data;
use crate::x_data;

use crate::enums::*;
use enum_primitive::FromPrimitive;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct AppId {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
}

impl Default for AppId {
    fn default() -> Self {
        AppId {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
        }
    }
}

impl AppId {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockRecord {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    #[doc(hidden)]
    pub __layout_handle: Handle,
    pub insertion_units: Units,
    pub explodability: bool,
    pub scalability: bool,
    #[doc(hidden)]
    pub __bitmap_preview_data: Vec<Vec<u8>>,
}

impl Default for BlockRecord {
    fn default() -> Self {
        BlockRecord {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            __layout_handle: Handle::empty(),
            insertion_units: Units::Unitless,
            explodability: true,
            scalability: true,
            __bitmap_preview_data: vec![],
        }
    }
}

impl BlockRecord {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DimStyle {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub dimensioning_suffix: String,
    pub alternate_dimensioning_suffix: String,
    pub first_arrow_block_name: String,
    pub second_arrow_block_name: String,
    pub dimensioning_scale_factor: f64,
    pub dimensioning_arrow_size: f64,
    pub dimension_extension_line_offset: f64,
    pub dimension_line_increment: f64,
    pub dimension_extension_line_extension: f64,
    pub dimension_distance_rounding_value: f64,
    pub dimension_line_extension: f64,
    pub dimension_plus_tolerance: f64,
    pub dimension_minus_tolerance: f64,
    pub generate_dimension_tolerances: bool,
    pub generate_dimension_limits: bool,
    pub dimension_text_inside_horizontal: bool,
    pub dimension_text_outside_horizontal: bool,
    pub suppress_first_dimension_extension_line: bool,
    pub suppress_second_dimension_extension_line: bool,
    pub text_above_dimension_line: bool,
    pub dimension_unit_zero_suppression: UnitZeroSuppression,
    pub dimension_angle_zero_suppression: UnitZeroSuppression,
    pub dimensioning_text_height: f64,
    pub center_mark_size: f64,
    pub dimensioning_tick_size: f64,
    pub alternate_dimensioning_scale_factor: f64,
    pub dimension_linear_measurement_scale_factor: f64,
    pub dimension_vertical_text_position: f64,
    pub dimension_tolerance_displace_scale_factor: f64,
    pub dimension_line_gap: f64,
    pub alternate_dimensioning_unit_rounding: f64,
    pub use_alternate_dimensioning: bool,
    pub alternate_dimensioning_decimal_places: i16,
    pub force_dimension_line_extensions_outside_if_text_exists: bool,
    pub use_separate_arrow_blocks_for_dimensions: bool,
    pub force_dimension_text_inside_extensions: bool,
    pub suppress_outside_extension_dimension_lines: bool,
    pub dimension_line_color: Color,
    pub dimension_extension_line_color: Color,
    pub dimension_text_color: Color,
    pub angular_dimension_precision: i16,
    pub dimension_unit_format: UnitFormat,
    pub dimension_unit_tolerance_decimal_places: i16,
    pub dimension_tolerace_decimal_places: i16,
    pub alternate_dimensioning_units: UnitFormat,
    pub alternate_dimensioning_tolerance_decimal_places: i16,
    pub dimensioning_angle_format: AngleFormat,
    pub dimension_precision: i16,
    pub dimension_non_angular_units: NonAngularUnits,
    pub dimension_decilam_separator_char: char,
    pub dimension_text_movement_rule: DimensionTextMovementRule,
    pub dimension_text_justification: DimensionTextJustification,
    pub dimension_tolerance_vertical_justification: Justification,
    pub dimension_tolerance_zero_suppression: UnitZeroSuppression,
    pub alternate_dimensioning_zero_suppression: UnitZeroSuppression,
    pub alternate_dimensioning_tolerance_zero_suppression: UnitZeroSuppression,
    pub dimension_text_and_arrow_placement: DimensionFit,
    pub dimension_cursor_controls_text_position: bool,
    pub dimension_text_style: String,
    pub dimension_leader_block_name: String,
    pub arrow_block_name: String,
    pub dimension_line_weight: LineWeight,
    pub dimension_extension_line_weight: LineWeight,
}

impl Default for DimStyle {
    fn default() -> Self {
        DimStyle {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            dimensioning_suffix: String::new(),
            alternate_dimensioning_suffix: String::new(),
            first_arrow_block_name: String::new(),
            second_arrow_block_name: String::new(),
            dimensioning_scale_factor: 1.0,
            dimensioning_arrow_size: 0.18,
            dimension_extension_line_offset: 0.0625,
            dimension_line_increment: 0.38,
            dimension_extension_line_extension: 0.18,
            dimension_distance_rounding_value: 0.0,
            dimension_line_extension: 0.0,
            dimension_plus_tolerance: 0.0,
            dimension_minus_tolerance: 0.0,
            generate_dimension_tolerances: false,
            generate_dimension_limits: false,
            dimension_text_inside_horizontal: true,
            dimension_text_outside_horizontal: true,
            suppress_first_dimension_extension_line: false,
            suppress_second_dimension_extension_line: false,
            text_above_dimension_line: false,
            dimension_unit_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches,
            dimension_angle_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches,
            dimensioning_text_height: 0.18,
            center_mark_size: 0.09,
            dimensioning_tick_size: 0.0,
            alternate_dimensioning_scale_factor: 25.4,
            dimension_linear_measurement_scale_factor: 1.0,
            dimension_vertical_text_position: 0.0,
            dimension_tolerance_displace_scale_factor: 1.0,
            dimension_line_gap: 0.09,
            alternate_dimensioning_unit_rounding: 0.0,
            use_alternate_dimensioning: false,
            alternate_dimensioning_decimal_places: 2,
            force_dimension_line_extensions_outside_if_text_exists: false,
            use_separate_arrow_blocks_for_dimensions: false,
            force_dimension_text_inside_extensions: false,
            suppress_outside_extension_dimension_lines: false,
            dimension_line_color: Color::by_block(),
            dimension_extension_line_color: Color::by_block(),
            dimension_text_color: Color::by_block(),
            angular_dimension_precision: 12,
            dimension_unit_format: UnitFormat::Scientific,
            dimension_unit_tolerance_decimal_places: 0,
            dimension_tolerace_decimal_places: 0,
            alternate_dimensioning_units: UnitFormat::Scientific,
            alternate_dimensioning_tolerance_decimal_places: 0,
            dimensioning_angle_format: AngleFormat::DecimalDegrees,
            dimension_precision: 12,
            dimension_non_angular_units: NonAngularUnits::Scientific,
            dimension_decilam_separator_char: '.',
            dimension_text_movement_rule: DimensionTextMovementRule::MoveLineWithText,
            dimension_text_justification: DimensionTextJustification::AboveLineCenter,
            dimension_tolerance_vertical_justification: Justification::Top,
            dimension_tolerance_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches,
            alternate_dimensioning_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches,
            alternate_dimensioning_tolerance_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches,
            dimension_text_and_arrow_placement: DimensionFit::TextAndArrowsOutsideLines,
            dimension_cursor_controls_text_position: true,
            dimension_text_style: String::new(),
            dimension_leader_block_name: String::new(),
            arrow_block_name: String::new(),
            dimension_line_weight: LineWeight::default(),
            dimension_extension_line_weight: LineWeight::default(),
        }
    }
}

impl DimStyle {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LineType {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub description: String,
    pub alignment_code: i32,
    pub element_count: i32,
    pub total_pattern_length: f64,
    pub dash_dot_space_lengths: Vec<f64>,
    pub complex_line_type_element_types: Vec<i16>,
    pub shape_numbers: Vec<i16>,
    #[doc(hidden)]
    pub __styles_handle: Vec<Handle>,
    pub scale_values: Vec<f64>,
    pub rotation_angles: Vec<f64>,
    pub x_offsets: Vec<f64>,
    pub y_offsets: Vec<f64>,
    pub text_strings: Vec<String>,
}

impl Default for LineType {
    fn default() -> Self {
        LineType {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            description: String::new(),
            alignment_code: 'A' as i32,
            element_count: 0,
            total_pattern_length: 0.0,
            dash_dot_space_lengths: vec![],
            complex_line_type_element_types: vec![],
            shape_numbers: vec![],
            __styles_handle: vec![],
            scale_values: vec![],
            rotation_angles: vec![],
            x_offsets: vec![],
            y_offsets: vec![],
            text_strings: vec![],
        }
    }
}

impl LineType {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Layer {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub color: Color,
    pub line_type_name: String,
    pub is_layer_plotted: bool,
    pub line_weight: LineWeight,
    #[doc(hidden)]
    pub __plot_style_handle: Handle,
    #[doc(hidden)]
    pub __material_handle: Handle,
    pub is_layer_on: bool,
}

impl Default for Layer {
    fn default() -> Self {
        Layer {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            color: Color::from_index(7),
            line_type_name: String::from("CONTINUOUS"),
            is_layer_plotted: true,
            line_weight: LineWeight::default(),
            __plot_style_handle: Handle::empty(),
            __material_handle: Handle::empty(),
            is_layer_on: true,
        }
    }
}

impl Layer {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub text_height: f64,
    pub width_factor: f64,
    pub oblique_angle: f64,
    pub text_generation_flags: i32,
    pub last_height_used: f64,
    pub primary_font_file_name: String,
    pub big_font_file_name: String,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            text_height: 0.0,
            width_factor: 1.0,
            oblique_angle: 0.0,
            text_generation_flags: 0,
            last_height_used: 0.2,
            primary_font_file_name: String::from("txt"),
            big_font_file_name: String::new(),
        }
    }
}

impl Style {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Ucs {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub origin: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub orthographic_view_type: OrthographicViewType,
    pub elevation: f64,
    #[doc(hidden)]
    pub __base_ucs_handle: Handle,
    pub orthographic_type: OrthographicViewType,
    pub orthographic_origin: Point,
}

impl Default for Ucs {
    fn default() -> Self {
        Ucs {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            origin: Point::origin(),
            x_axis: Vector::x_axis(),
            y_axis: Vector::y_axis(),
            orthographic_view_type: OrthographicViewType::None,
            elevation: 0.0,
            __base_ucs_handle: Handle::empty(),
            orthographic_type: OrthographicViewType::Top,
            orthographic_origin: Point::origin(),
        }
    }
}

impl Ucs {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct View {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub view_height: f64,
    pub view_center_point: Point,
    pub view_width: f64,
    pub view_direction: Vector,
    pub target_point: Point,
    pub lens_length: f64,
    pub front_clipping_plane: f64,
    pub back_clipping_plane: f64,
    pub twist_angle: f64,
    pub view_mode: i16,
    pub render_mode: ViewRenderMode,
    pub is_associated_ucs_present: bool,
    pub is_camera_plottable: bool,
    #[doc(hidden)]
    pub __background_object_handle: Handle,
    #[doc(hidden)]
    pub __selection_object_handle: Handle,
    #[doc(hidden)]
    pub __visual_style_object_handle: Handle,
    #[doc(hidden)]
    pub __sun_ownership_handle: Handle,
    pub ucs_origin: Point,
    pub ucs_x_axis: Vector,
    pub ucs_y_axis: Vector,
    pub orthographic_view_type: OrthographicViewType,
    pub ucs_elevation: f64,
    #[doc(hidden)]
    pub __ucs_handle: Handle,
    #[doc(hidden)]
    pub __base_ucs_handle: Handle,
}

impl Default for View {
    fn default() -> Self {
        View {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            view_height: 1.0,
            view_center_point: Point::origin(),
            view_width: 1.0,
            view_direction: Vector::z_axis(),
            target_point: Point::origin(),
            lens_length: 1.0,
            front_clipping_plane: 0.0,
            back_clipping_plane: 1.0,
            twist_angle: 0.0,
            view_mode: 0,
            render_mode: ViewRenderMode::Classic2D,
            is_associated_ucs_present: false,
            is_camera_plottable: false,
            __background_object_handle: Handle::empty(),
            __selection_object_handle: Handle::empty(),
            __visual_style_object_handle: Handle::empty(),
            __sun_ownership_handle: Handle::empty(),
            ucs_origin: Point::origin(),
            ucs_x_axis: Vector::x_axis(),
            ucs_y_axis: Vector::y_axis(),
            orthographic_view_type: OrthographicViewType::None,
            ucs_elevation: 0.0,
            __ucs_handle: Handle::empty(),
            __base_ucs_handle: Handle::empty(),
        }
    }
}

impl View {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewPort {
    pub name: String,
    pub handle: Handle,
    #[doc(hidden)]
    pub __owner_handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    pub lower_left: Point,
    pub upper_right: Point,
    pub view_center: Point,
    pub snap_base_point: Point,
    pub snap_spacing: Vector,
    pub grid_spacing: Vector,
    pub view_direction: Vector,
    pub target_view_point: Point,
    pub view_height: f64,
    pub view_port_aspect_ratio: f64,
    pub lens_length: f64,
    pub front_clipping_plane: f64,
    pub back_clipping_plane: f64,
    pub snap_rotation_angle: f64,
    pub view_twist_angle: f64,
    pub view_mode: ViewMode,
    pub circle_sides: i32,
    pub fast_zoom: bool,
    pub ucs_icon: i16,
    pub snap_on: bool,
    pub grid_on: bool,
    pub snap_style: SnapStyle,
    pub snap_isometric_plane: SnapIsometricPlane,
    pub plot_style_sheet: String,
    pub render_mode: ViewRenderMode,
    pub has_own_ucs: bool,
    pub ucs_origin: Point,
    pub ucs_x_axis: Vector,
    pub ucs_y_axis: Vector,
    pub orthographic_view_type: OrthographicViewType,
    pub ucs_elevation: f64,
    #[doc(hidden)]
    pub __ucs_handle: Handle,
    #[doc(hidden)]
    pub __base_ucs_handle: Handle,
    pub shade_plot_setting: ShadeEdgeMode,
    pub major_grid_lines: bool,
    #[doc(hidden)]
    pub __background_object_handle: Handle,
    #[doc(hidden)]
    pub __shade_plot_object_handle: Handle,
    #[doc(hidden)]
    pub __visual_style_object_handle: Handle,
    pub is_default_lighting_on: bool,
    pub default_lighting_type: DefaultLightingType,
    pub brightness: f64,
    pub contrast: f64,
    pub ambient_color: Color,
    pub ambient_color_i32: i32,
    pub ambient_color_name: String,
}

impl Default for ViewPort {
    fn default() -> Self {
        ViewPort {
            name: String::new(),
            handle: Handle::empty(),
            __owner_handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            lower_left: Point::origin(),
            upper_right: Point::new(1.0, 1.0, 0.0),
            view_center: Point::origin(),
            snap_base_point: Point::origin(),
            snap_spacing: Vector::new(1.0, 1.0, 0.0),
            grid_spacing: Vector::new(1.0, 1.0, 0.0),
            view_direction: Vector::z_axis(),
            target_view_point: Point::origin(),
            view_height: 1.0,
            view_port_aspect_ratio: 1.0,
            lens_length: 50.0,
            front_clipping_plane: 0.0,
            back_clipping_plane: 0.0,
            snap_rotation_angle: 0.0,
            view_twist_angle: 0.0,
            view_mode: ViewMode::default(),
            circle_sides: 1000,
            fast_zoom: true,
            ucs_icon: 3,
            snap_on: false,
            grid_on: false,
            snap_style: SnapStyle::Standard,
            snap_isometric_plane: SnapIsometricPlane::Left,
            plot_style_sheet: String::new(),
            render_mode: ViewRenderMode::Classic2D,
            has_own_ucs: false,
            ucs_origin: Point::origin(),
            ucs_x_axis: Vector::x_axis(),
            ucs_y_axis: Vector::y_axis(),
            orthographic_view_type: OrthographicViewType::None,
            ucs_elevation: 0.0,
            __ucs_handle: Handle::empty(),
            __base_ucs_handle: Handle::empty(),
            shade_plot_setting: ShadeEdgeMode::FacesShadedEdgeNotHighlighted,
            major_grid_lines: false,
            __background_object_handle: Handle::empty(),
            __shade_plot_object_handle: Handle::empty(),
            __visual_style_object_handle: Handle::empty(),
            is_default_lighting_on: true,
            default_lighting_type: DefaultLightingType::OneDistantLight,
            brightness: 0.0,
            contrast: 0.0,
            ambient_color: Color::from_raw_value(7),
            ambient_color_i32: 0,
            ambient_color_name: String::from("BLACK"),
        }
    }
}

impl ViewPort {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner<'a>(&mut self, item: &'a mut DrawingItemMut, drawing: &'a mut Drawing) {
        self.__owner_handle = drawing.assign_and_get_handle(item);
    }
}

pub(crate) fn read_specific_table(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    match iter.next() {
        Some(Ok(pair)) => {
            if pair.code != 2 {
                return Err(DxfError::ExpectedTableType(pair.offset));
            }

            match &*pair.assert_string()? {
                "APPID" => read_app_ids(drawing, iter)?,
                "BLOCK_RECORD" => read_block_records(drawing, iter)?,
                "DIMSTYLE" => read_dim_styles(drawing, iter)?,
                "LTYPE" => read_line_types(drawing, iter)?,
                "LAYER" => read_layers(drawing, iter)?,
                "STYLE" => read_styles(drawing, iter)?,
                "UCS" => read_ucss(drawing, iter)?,
                "VIEW" => read_views(drawing, iter)?,
                "VPORT" => read_view_ports(drawing, iter)?,
                _ => Drawing::swallow_table(iter)?,
            }

            match iter.next() {
                Some(Ok(CodePair { code: 0, value: CodePairValue::Str(ref s), .. })) if s == "ENDTAB" => (),
                Some(Ok(pair)) => return Err(DxfError::UnexpectedCodePair(pair, String::from("expected 0/ENDTAB"))),
                Some(Err(e)) => return Err(e),
                None => return Err(DxfError::UnexpectedEndOfInput),
            }
        },
        Some(Err(e)) => return Err(e),
        None => return Err(DxfError::UnexpectedEndOfInput),
    }

    Ok(())
}

fn read_app_ids(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "APPID" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = AppId::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_app_id(item);
                    }
                    else {
                        drawing.add_app_id_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_block_records(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "BLOCK_RECORD" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = BlockRecord::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    340 => { item.__layout_handle = pair.as_handle()?; },
                                    70 => { item.insertion_units = enum_from_number!(Units, Unitless, from_i16, pair.assert_i16()?); },
                                    280 => { item.explodability = as_bool(pair.assert_i16()?); },
                                    281 => { item.scalability = as_bool(pair.assert_i16()?); },
                                    310 => { item.__bitmap_preview_data.push(pair.assert_binary()?); },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_block_record(item);
                    }
                    else {
                        drawing.add_block_record_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_dim_styles(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "DIMSTYLE" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = DimStyle::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    3 => { item.dimensioning_suffix = pair.assert_string()?; },
                                    4 => { item.alternate_dimensioning_suffix = pair.assert_string()?; },
                                    6 => { item.first_arrow_block_name = pair.assert_string()?; },
                                    7 => { item.second_arrow_block_name = pair.assert_string()?; },
                                    40 => { item.dimensioning_scale_factor = pair.assert_f64()?; },
                                    41 => { item.dimensioning_arrow_size = pair.assert_f64()?; },
                                    42 => { item.dimension_extension_line_offset = pair.assert_f64()?; },
                                    43 => { item.dimension_line_increment = pair.assert_f64()?; },
                                    44 => { item.dimension_extension_line_extension = pair.assert_f64()?; },
                                    45 => { item.dimension_distance_rounding_value = pair.assert_f64()?; },
                                    46 => { item.dimension_line_extension = pair.assert_f64()?; },
                                    47 => { item.dimension_plus_tolerance = pair.assert_f64()?; },
                                    48 => { item.dimension_minus_tolerance = pair.assert_f64()?; },
                                    71 => { item.generate_dimension_tolerances = as_bool(pair.assert_i16()?); },
                                    72 => { item.generate_dimension_limits = as_bool(pair.assert_i16()?); },
                                    73 => { item.dimension_text_inside_horizontal = as_bool(pair.assert_i16()?); },
                                    74 => { item.dimension_text_outside_horizontal = as_bool(pair.assert_i16()?); },
                                    75 => { item.suppress_first_dimension_extension_line = as_bool(pair.assert_i16()?); },
                                    76 => { item.suppress_second_dimension_extension_line = as_bool(pair.assert_i16()?); },
                                    77 => { item.text_above_dimension_line = as_bool(pair.assert_i16()?); },
                                    78 => { item.dimension_unit_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
                                    79 => { item.dimension_angle_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
                                    140 => { item.dimensioning_text_height = pair.assert_f64()?; },
                                    141 => { item.center_mark_size = pair.assert_f64()?; },
                                    142 => { item.dimensioning_tick_size = pair.assert_f64()?; },
                                    143 => { item.alternate_dimensioning_scale_factor = pair.assert_f64()?; },
                                    144 => { item.dimension_linear_measurement_scale_factor = pair.assert_f64()?; },
                                    145 => { item.dimension_vertical_text_position = pair.assert_f64()?; },
                                    146 => { item.dimension_tolerance_displace_scale_factor = pair.assert_f64()?; },
                                    147 => { item.dimension_line_gap = pair.assert_f64()?; },
                                    148 => { item.alternate_dimensioning_unit_rounding = pair.assert_f64()?; },
                                    170 => { item.use_alternate_dimensioning = as_bool(pair.assert_i16()?); },
                                    171 => { item.alternate_dimensioning_decimal_places = pair.assert_i16()?; },
                                    172 => { item.force_dimension_line_extensions_outside_if_text_exists = as_bool(pair.assert_i16()?); },
                                    173 => { item.use_separate_arrow_blocks_for_dimensions = as_bool(pair.assert_i16()?); },
                                    174 => { item.force_dimension_text_inside_extensions = as_bool(pair.assert_i16()?); },
                                    175 => { item.suppress_outside_extension_dimension_lines = as_bool(pair.assert_i16()?); },
                                    176 => { item.dimension_line_color = Color::from_raw_value(pair.assert_i16()?); },
                                    177 => { item.dimension_extension_line_color = Color::from_raw_value(pair.assert_i16()?); },
                                    178 => { item.dimension_text_color = Color::from_raw_value(pair.assert_i16()?); },
                                    179 => { item.angular_dimension_precision = pair.assert_i16()?; },
                                    270 => { item.dimension_unit_format = enum_from_number!(UnitFormat, Scientific, from_i16, pair.assert_i16()?); },
                                    271 => { item.dimension_unit_tolerance_decimal_places = pair.assert_i16()?; },
                                    272 => { item.dimension_tolerace_decimal_places = pair.assert_i16()?; },
                                    273 => { item.alternate_dimensioning_units = enum_from_number!(UnitFormat, Scientific, from_i16, pair.assert_i16()?); },
                                    274 => { item.alternate_dimensioning_tolerance_decimal_places = pair.assert_i16()?; },
                                    275 => { item.dimensioning_angle_format = enum_from_number!(AngleFormat, DecimalDegrees, from_i16, pair.assert_i16()?); },
                                    276 => { item.dimension_precision = pair.assert_i16()?; },
                                    277 => { item.dimension_non_angular_units = enum_from_number!(NonAngularUnits, Scientific, from_i16, pair.assert_i16()?); },
                                    278 => { item.dimension_decilam_separator_char = pair.assert_i16()? as u8 as char; },
                                    279 => { item.dimension_text_movement_rule = enum_from_number!(DimensionTextMovementRule, MoveLineWithText, from_i16, pair.assert_i16()?); },
                                    280 => { item.dimension_text_justification = enum_from_number!(DimensionTextJustification, AboveLineCenter, from_i16, pair.assert_i16()?); },
                                    281 => { item.suppress_first_dimension_extension_line = as_bool(pair.assert_i16()?); },
                                    282 => { item.suppress_second_dimension_extension_line = as_bool(pair.assert_i16()?); },
                                    283 => { item.dimension_tolerance_vertical_justification = enum_from_number!(Justification, Top, from_i16, pair.assert_i16()?); },
                                    284 => { item.dimension_tolerance_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
                                    285 => { item.alternate_dimensioning_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
                                    286 => { item.alternate_dimensioning_tolerance_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
                                    287 => { item.dimension_text_and_arrow_placement = enum_from_number!(DimensionFit, TextAndArrowsOutsideLines, from_i16, pair.assert_i16()?); },
                                    288 => { item.dimension_cursor_controls_text_position = as_bool(pair.assert_i16()?); },
                                    289 => { item.dimension_text_and_arrow_placement = enum_from_number!(DimensionFit, TextAndArrowsOutsideLines, from_i16, pair.assert_i16()?); },
                                    340 => { item.dimension_text_style = pair.assert_string()?; },
                                    341 => { item.dimension_leader_block_name = pair.assert_string()?; },
                                    342 => { item.arrow_block_name = pair.assert_string()?; },
                                    343 => { item.first_arrow_block_name = pair.assert_string()?; },
                                    344 => { item.second_arrow_block_name = pair.assert_string()?; },
                                    371 => { item.dimension_line_weight = LineWeight::from_raw_value(pair.assert_i16()?); },
                                    372 => { item.dimension_extension_line_weight = LineWeight::from_raw_value(pair.assert_i16()?); },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_dim_style(item);
                    }
                    else {
                        drawing.add_dim_style_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_line_types(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "LTYPE" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = LineType::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    3 => { item.description = pair.assert_string()?; },
                                    72 => { item.alignment_code = i32::from(pair.assert_i16()?); },
                                    73 => { item.element_count = i32::from(pair.assert_i16()?); },
                                    40 => { item.total_pattern_length = pair.assert_f64()?; },
                                    49 => { item.dash_dot_space_lengths.push(pair.assert_f64()?); },
                                    74 => { item.complex_line_type_element_types.push(pair.assert_i16()?); },
                                    75 => { item.shape_numbers.push(pair.assert_i16()?); },
                                    340 => { item.__styles_handle.push(pair.as_handle()?); },
                                    46 => { item.scale_values.push(pair.assert_f64()?); },
                                    50 => { item.rotation_angles.push(pair.assert_f64()?); },
                                    44 => { item.x_offsets.push(pair.assert_f64()?); },
                                    45 => { item.y_offsets.push(pair.assert_f64()?); },
                                    9 => { item.text_strings.push(pair.assert_string()?); },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_line_type(item);
                    }
                    else {
                        drawing.add_line_type_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_layers(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "LAYER" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = Layer::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    62 => { item.color = read_color_value(&mut item, pair.assert_i16()?); },
                                    6 => { item.line_type_name = pair.assert_string()?; },
                                    290 => { item.is_layer_plotted = pair.assert_bool()?; },
                                    370 => { item.line_weight = LineWeight::from_raw_value(pair.assert_i16()?); },
                                    390 => { item.__plot_style_handle = pair.as_handle()?; },
                                    347 => { item.__material_handle = pair.as_handle()?; },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_layer(item);
                    }
                    else {
                        drawing.add_layer_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_styles(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "STYLE" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = Style::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    40 => { item.text_height = pair.assert_f64()?; },
                                    41 => { item.width_factor = pair.assert_f64()?; },
                                    50 => { item.oblique_angle = pair.assert_f64()?; },
                                    71 => { item.text_generation_flags = i32::from(pair.assert_i16()?); },
                                    42 => { item.last_height_used = pair.assert_f64()?; },
                                    3 => { item.primary_font_file_name = pair.assert_string()?; },
                                    4 => { item.big_font_file_name = pair.assert_string()?; },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_style(item);
                    }
                    else {
                        drawing.add_style_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_ucss(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "UCS" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = Ucs::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    10 => { item.origin.x = pair.assert_f64()?; },
                                    20 => { item.origin.y = pair.assert_f64()?; },
                                    30 => { item.origin.z = pair.assert_f64()?; },
                                    11 => { item.x_axis.x = pair.assert_f64()?; },
                                    21 => { item.x_axis.y = pair.assert_f64()?; },
                                    31 => { item.x_axis.z = pair.assert_f64()?; },
                                    12 => { item.y_axis.x = pair.assert_f64()?; },
                                    22 => { item.y_axis.y = pair.assert_f64()?; },
                                    32 => { item.y_axis.z = pair.assert_f64()?; },
                                    79 => { item.orthographic_view_type = enum_from_number!(OrthographicViewType, None, from_i16, pair.assert_i16()?); },
                                    146 => { item.elevation = pair.assert_f64()?; },
                                    346 => { item.__base_ucs_handle = pair.as_handle()?; },
                                    71 => { item.orthographic_type = enum_from_number!(OrthographicViewType, Top, from_i16, pair.assert_i16()?); },
                                    13 => { item.orthographic_origin.x = pair.assert_f64()?; },
                                    23 => { item.orthographic_origin.y = pair.assert_f64()?; },
                                    33 => { item.orthographic_origin.z = pair.assert_f64()?; },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_ucs(item);
                    }
                    else {
                        drawing.add_ucs_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_views(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "VIEW" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = View::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    40 => { item.view_height = pair.assert_f64()?; },
                                    10 => { item.view_center_point.x = pair.assert_f64()?; },
                                    20 => { item.view_center_point.y = pair.assert_f64()?; },
                                    41 => { item.view_width = pair.assert_f64()?; },
                                    11 => { item.view_direction.x = pair.assert_f64()?; },
                                    21 => { item.view_direction.y = pair.assert_f64()?; },
                                    31 => { item.view_direction.z = pair.assert_f64()?; },
                                    12 => { item.target_point.x = pair.assert_f64()?; },
                                    22 => { item.target_point.y = pair.assert_f64()?; },
                                    32 => { item.target_point.z = pair.assert_f64()?; },
                                    42 => { item.lens_length = pair.assert_f64()?; },
                                    43 => { item.front_clipping_plane = pair.assert_f64()?; },
                                    44 => { item.back_clipping_plane = pair.assert_f64()?; },
                                    50 => { item.twist_angle = pair.assert_f64()?; },
                                    71 => { item.view_mode = pair.assert_i16()?; },
                                    281 => { item.render_mode = enum_from_number!(ViewRenderMode, Classic2D, from_i16, pair.assert_i16()?); },
                                    72 => { item.is_associated_ucs_present = as_bool(pair.assert_i16()?); },
                                    73 => { item.is_camera_plottable = as_bool(pair.assert_i16()?); },
                                    332 => { item.__background_object_handle = pair.as_handle()?; },
                                    334 => { item.__selection_object_handle = pair.as_handle()?; },
                                    338 => { item.__visual_style_object_handle = pair.as_handle()?; },
                                    361 => { item.__sun_ownership_handle = pair.as_handle()?; },
                                    110 => { item.ucs_origin.x = pair.assert_f64()?; },
                                    120 => { item.ucs_origin.y = pair.assert_f64()?; },
                                    130 => { item.ucs_origin.z = pair.assert_f64()?; },
                                    111 => { item.ucs_x_axis.x = pair.assert_f64()?; },
                                    121 => { item.ucs_x_axis.y = pair.assert_f64()?; },
                                    131 => { item.ucs_x_axis.z = pair.assert_f64()?; },
                                    112 => { item.ucs_y_axis.x = pair.assert_f64()?; },
                                    122 => { item.ucs_y_axis.y = pair.assert_f64()?; },
                                    132 => { item.ucs_y_axis.z = pair.assert_f64()?; },
                                    79 => { item.orthographic_view_type = enum_from_number!(OrthographicViewType, None, from_i16, pair.assert_i16()?); },
                                    146 => { item.ucs_elevation = pair.assert_f64()?; },
                                    345 => { item.__ucs_handle = pair.as_handle()?; },
                                    346 => { item.__base_ucs_handle = pair.as_handle()?; },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_view(item);
                    }
                    else {
                        drawing.add_view_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

fn read_view_ports(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
    loop {
        match iter.next() {
            Some(Ok(pair)) => {
                if pair.code == 0 {
                    if pair.assert_string()? != "VPORT" {
                        iter.put_back(Ok(pair));
                        break;
                    }

                    let mut item = ViewPort::default();
                    loop {
                        match iter.next() {
                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                iter.put_back(Ok(pair));
                                break;
                            },
                            Some(Ok(pair)) => {
                                match pair.code {
                                    2 => item.name = pair.assert_string()?,
                                    5 => item.handle = pair.as_handle()?,
                                    extension_data::EXTENSION_DATA_GROUP => {
                                        let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                                        item.extension_data_groups.push(group);
                                    },
                                    x_data::XDATA_APPLICATIONNAME => {
                                        let x = XData::read_item(pair.assert_string()?, iter)?;
                                        item.x_data.push(x);
                                    },
                                    330 => item.__owner_handle = pair.as_handle()?,
                                    10 => { item.lower_left.x = pair.assert_f64()?; },
                                    20 => { item.lower_left.y = pair.assert_f64()?; },
                                    11 => { item.upper_right.x = pair.assert_f64()?; },
                                    21 => { item.upper_right.y = pair.assert_f64()?; },
                                    12 => { item.view_center.x = pair.assert_f64()?; },
                                    22 => { item.view_center.y = pair.assert_f64()?; },
                                    13 => { item.snap_base_point.x = pair.assert_f64()?; },
                                    23 => { item.snap_base_point.y = pair.assert_f64()?; },
                                    14 => { item.snap_spacing.x = pair.assert_f64()?; },
                                    24 => { item.snap_spacing.y = pair.assert_f64()?; },
                                    15 => { item.grid_spacing.x = pair.assert_f64()?; },
                                    25 => { item.grid_spacing.y = pair.assert_f64()?; },
                                    16 => { item.view_direction.x = pair.assert_f64()?; },
                                    26 => { item.view_direction.y = pair.assert_f64()?; },
                                    36 => { item.view_direction.z = pair.assert_f64()?; },
                                    17 => { item.target_view_point.x = pair.assert_f64()?; },
                                    27 => { item.target_view_point.y = pair.assert_f64()?; },
                                    37 => { item.target_view_point.z = pair.assert_f64()?; },
                                    40 => { item.view_height = pair.assert_f64()?; },
                                    41 => { item.view_port_aspect_ratio = pair.assert_f64()?; },
                                    42 => { item.lens_length = pair.assert_f64()?; },
                                    43 => { item.front_clipping_plane = pair.assert_f64()?; },
                                    44 => { item.back_clipping_plane = pair.assert_f64()?; },
                                    45 => { item.view_height = pair.assert_f64()?; },
                                    50 => { item.snap_rotation_angle = pair.assert_f64()?; },
                                    51 => { item.view_twist_angle = pair.assert_f64()?; },
                                    71 => { item.view_mode = ViewMode::from_i16(pair.assert_i16()?); },
                                    72 => { item.circle_sides = i32::from(pair.assert_i16()?); },
                                    73 => { item.fast_zoom = as_bool(pair.assert_i16()?); },
                                    74 => { item.ucs_icon = pair.assert_i16()?; },
                                    75 => { item.snap_on = as_bool(pair.assert_i16()?); },
                                    76 => { item.grid_on = as_bool(pair.assert_i16()?); },
                                    77 => { item.snap_style = enum_from_number!(SnapStyle, Standard, from_i16, pair.assert_i16()?); },
                                    78 => { item.snap_isometric_plane = enum_from_number!(SnapIsometricPlane, Left, from_i16, pair.assert_i16()?); },
                                    1 => { item.plot_style_sheet = pair.assert_string()?; },
                                    281 => { item.render_mode = enum_from_number!(ViewRenderMode, Classic2D, from_i16, pair.assert_i16()?); },
                                    65 => { item.has_own_ucs = as_bool(pair.assert_i16()?); },
                                    110 => { item.ucs_origin.x = pair.assert_f64()?; },
                                    120 => { item.ucs_origin.y = pair.assert_f64()?; },
                                    130 => { item.ucs_origin.z = pair.assert_f64()?; },
                                    111 => { item.ucs_x_axis.x = pair.assert_f64()?; },
                                    121 => { item.ucs_x_axis.y = pair.assert_f64()?; },
                                    131 => { item.ucs_x_axis.z = pair.assert_f64()?; },
                                    112 => { item.ucs_y_axis.x = pair.assert_f64()?; },
                                    122 => { item.ucs_y_axis.y = pair.assert_f64()?; },
                                    132 => { item.ucs_y_axis.z = pair.assert_f64()?; },
                                    79 => { item.orthographic_view_type = enum_from_number!(OrthographicViewType, None, from_i16, pair.assert_i16()?); },
                                    146 => { item.ucs_elevation = pair.assert_f64()?; },
                                    345 => { item.__ucs_handle = pair.as_handle()?; },
                                    346 => { item.__base_ucs_handle = pair.as_handle()?; },
                                    170 => { item.shade_plot_setting = enum_from_number!(ShadeEdgeMode, FacesShadedEdgeNotHighlighted, from_i16, pair.assert_i16()?); },
                                    61 => { item.major_grid_lines = as_bool(pair.assert_i16()?); },
                                    332 => { item.__background_object_handle = pair.as_handle()?; },
                                    333 => { item.__shade_plot_object_handle = pair.as_handle()?; },
                                    348 => { item.__visual_style_object_handle = pair.as_handle()?; },
                                    292 => { item.is_default_lighting_on = pair.assert_bool()?; },
                                    282 => { item.default_lighting_type = enum_from_number!(DefaultLightingType, OneDistantLight, from_i16, pair.assert_i16()?); },
                                    141 => { item.brightness = pair.assert_f64()?; },
                                    142 => { item.contrast = pair.assert_f64()?; },
                                    62 => { item.ambient_color = Color::from_raw_value(pair.assert_i16()?); },
                                    421 => { item.ambient_color_i32 = pair.assert_i32()?; },
                                    431 => { item.ambient_color_name = pair.assert_string()?; },
                                    _ => (), // unsupported code
                                }
                            },
                            Some(Err(e)) => return Err(e),
                            None => return Err(DxfError::UnexpectedEndOfInput),
                        }
                    }

                    if item.handle.is_empty() {
                        drawing.add_view_port(item);
                    }
                    else {
                        drawing.add_view_port_no_handle_set(item);
                    }
                }
                else {
                    // do nothing, probably the table's handle or flags
                }
            },
            Some(Err(e)) => return Err(e),
            None => return Err(DxfError::UnexpectedEndOfInput),
        }
    }

    Ok(())
}

pub(crate) fn add_table_code_pairs(drawing: &Drawing, pairs: &mut Vec<CodePair>, write_handles: bool) {
    add_app_ids_code_pairs(pairs, drawing, write_handles);
    if drawing.header.version >= AcadVersion::R13 {
        add_block_records_code_pairs(pairs, drawing, write_handles);
    }
    add_dim_styles_code_pairs(pairs, drawing, write_handles);
    add_line_types_code_pairs(pairs, drawing, write_handles);
    add_layers_code_pairs(pairs, drawing, write_handles);
    add_styles_code_pairs(pairs, drawing, write_handles);
    add_ucss_code_pairs(pairs, drawing, write_handles);
    add_views_code_pairs(pairs, drawing, write_handles);
    add_view_ports_code_pairs(pairs, drawing, write_handles);
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_app_ids_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.app_ids().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "APPID"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.app_ids() {
        pairs.push(CodePair::new_str(0, "APPID"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::AppId(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbRegAppTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_block_records_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.block_records().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "BLOCK_RECORD"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.block_records() {
        pairs.push(CodePair::new_str(0, "BLOCK_RECORD"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::BlockRecord(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbBlockTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(340, &item.__layout_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(70, item.insertion_units as i16));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(280, as_i16(item.explodability)));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(281, as_i16(item.scalability)));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            for x in &item.__bitmap_preview_data {
                pairs.push(CodePair::new_binary(310, x.clone()));
            }
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_dim_styles_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.dim_styles().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "DIMSTYLE"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.dim_styles() {
        pairs.push(CodePair::new_str(0, "DIMSTYLE"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::DimStyle(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbDimStyleTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_string(3, &item.dimensioning_suffix));
        pairs.push(CodePair::new_string(4, &item.alternate_dimensioning_suffix));
        if drawing.header.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_string(6, &item.first_arrow_block_name));
        }
        if drawing.header.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_string(7, &item.second_arrow_block_name));
        }
        pairs.push(CodePair::new_f64(40, item.dimensioning_scale_factor));
        pairs.push(CodePair::new_f64(41, item.dimensioning_arrow_size));
        pairs.push(CodePair::new_f64(42, item.dimension_extension_line_offset));
        pairs.push(CodePair::new_f64(43, item.dimension_line_increment));
        pairs.push(CodePair::new_f64(44, item.dimension_extension_line_extension));
        pairs.push(CodePair::new_f64(45, item.dimension_distance_rounding_value));
        pairs.push(CodePair::new_f64(46, item.dimension_line_extension));
        pairs.push(CodePair::new_f64(47, item.dimension_plus_tolerance));
        pairs.push(CodePair::new_f64(48, item.dimension_minus_tolerance));
        pairs.push(CodePair::new_i16(71, as_i16(item.generate_dimension_tolerances)));
        pairs.push(CodePair::new_i16(72, as_i16(item.generate_dimension_limits)));
        pairs.push(CodePair::new_i16(73, as_i16(item.dimension_text_inside_horizontal)));
        pairs.push(CodePair::new_i16(74, as_i16(item.dimension_text_outside_horizontal)));
        pairs.push(CodePair::new_i16(75, as_i16(item.suppress_first_dimension_extension_line)));
        pairs.push(CodePair::new_i16(76, as_i16(item.suppress_second_dimension_extension_line)));
        pairs.push(CodePair::new_i16(77, as_i16(item.text_above_dimension_line)));
        pairs.push(CodePair::new_i16(78, item.dimension_unit_zero_suppression as i16));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(79, item.dimension_angle_zero_suppression as i16));
        }
        pairs.push(CodePair::new_f64(140, item.dimensioning_text_height));
        pairs.push(CodePair::new_f64(141, item.center_mark_size));
        pairs.push(CodePair::new_f64(142, item.dimensioning_tick_size));
        pairs.push(CodePair::new_f64(143, item.alternate_dimensioning_scale_factor));
        pairs.push(CodePair::new_f64(144, item.dimension_linear_measurement_scale_factor));
        pairs.push(CodePair::new_f64(145, item.dimension_vertical_text_position));
        pairs.push(CodePair::new_f64(146, item.dimension_tolerance_displace_scale_factor));
        pairs.push(CodePair::new_f64(147, item.dimension_line_gap));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_f64(148, item.alternate_dimensioning_unit_rounding));
        }
        pairs.push(CodePair::new_i16(170, as_i16(item.use_alternate_dimensioning)));
        pairs.push(CodePair::new_i16(171, item.alternate_dimensioning_decimal_places));
        pairs.push(CodePair::new_i16(172, as_i16(item.force_dimension_line_extensions_outside_if_text_exists)));
        pairs.push(CodePair::new_i16(173, as_i16(item.use_separate_arrow_blocks_for_dimensions)));
        pairs.push(CodePair::new_i16(174, as_i16(item.force_dimension_text_inside_extensions)));
        pairs.push(CodePair::new_i16(175, as_i16(item.suppress_outside_extension_dimension_lines)));
        pairs.push(CodePair::new_i16(176, Color::raw_value(&item.dimension_line_color)));
        pairs.push(CodePair::new_i16(177, Color::raw_value(&item.dimension_extension_line_color)));
        pairs.push(CodePair::new_i16(178, Color::raw_value(&item.dimension_text_color)));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(179, item.angular_dimension_precision));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(270, item.dimension_unit_format as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(271, item.dimension_unit_tolerance_decimal_places));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(272, item.dimension_tolerace_decimal_places));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(273, item.alternate_dimensioning_units as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(274, item.alternate_dimensioning_tolerance_decimal_places));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(275, item.dimensioning_angle_format as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(276, item.dimension_precision));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(277, item.dimension_non_angular_units as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(278, item.dimension_decilam_separator_char as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(279, item.dimension_text_movement_rule as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(280, item.dimension_text_justification as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(281, as_i16(item.suppress_first_dimension_extension_line)));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(282, as_i16(item.suppress_second_dimension_extension_line)));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(283, item.dimension_tolerance_vertical_justification as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(284, item.dimension_tolerance_zero_suppression as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(285, item.alternate_dimensioning_zero_suppression as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(286, item.alternate_dimensioning_tolerance_zero_suppression as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(287, item.dimension_text_and_arrow_placement as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(288, as_i16(item.dimension_cursor_controls_text_position)));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(289, item.dimension_text_and_arrow_placement as i16));
        }
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_string(340, &item.dimension_text_style));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(341, &item.dimension_leader_block_name));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(342, &item.arrow_block_name));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(343, &item.first_arrow_block_name));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(344, &item.second_arrow_block_name));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(371, LineWeight::raw_value(&item.dimension_line_weight)));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(372, LineWeight::raw_value(&item.dimension_extension_line_weight)));
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_line_types_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.line_types().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "LTYPE"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.line_types() {
        pairs.push(CodePair::new_str(0, "LTYPE"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::LineType(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbLinetypeTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_string(3, &item.description));
        pairs.push(CodePair::new_i16(72, item.alignment_code as i16));
        pairs.push(CodePair::new_i16(73, item.element_count as i16));
        pairs.push(CodePair::new_f64(40, item.total_pattern_length));
        for x in &item.dash_dot_space_lengths {
            pairs.push(CodePair::new_f64(49, *x));
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.complex_line_type_element_types {
                pairs.push(CodePair::new_i16(74, *x));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.shape_numbers {
                pairs.push(CodePair::new_i16(75, *x));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.__styles_handle {
                pairs.push(CodePair::new_string(340, &x.as_string()));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.scale_values {
                pairs.push(CodePair::new_f64(46, *x));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.rotation_angles {
                pairs.push(CodePair::new_f64(50, *x));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.x_offsets {
                pairs.push(CodePair::new_f64(44, *x));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.y_offsets {
                pairs.push(CodePair::new_f64(45, *x));
            }
        }
        if drawing.header.version >= AcadVersion::R13 {
            for x in &item.text_strings {
                pairs.push(CodePair::new_string(9, x));
            }
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_layers_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.layers().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "LAYER"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.layers() {
        pairs.push(CodePair::new_str(0, "LAYER"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::Layer(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbLayerTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_i16(62, item.color.writable_color_value(item)));
        pairs.push(CodePair::new_string(6, &item.line_type_name));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_bool(290, item.is_layer_plotted));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(370, LineWeight::raw_value(&item.line_weight)));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(390, &item.__plot_style_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(347, &item.__material_handle.as_string()));
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_styles_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.styles().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "STYLE"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.styles() {
        pairs.push(CodePair::new_str(0, "STYLE"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::Style(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbTextStyleTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_f64(40, item.text_height));
        pairs.push(CodePair::new_f64(41, item.width_factor));
        pairs.push(CodePair::new_f64(50, item.oblique_angle));
        pairs.push(CodePair::new_i16(71, item.text_generation_flags as i16));
        pairs.push(CodePair::new_f64(42, item.last_height_used));
        pairs.push(CodePair::new_string(3, &item.primary_font_file_name));
        pairs.push(CodePair::new_string(4, &item.big_font_file_name));
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_ucss_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.ucss().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "UCS"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.ucss() {
        pairs.push(CodePair::new_str(0, "UCS"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::Ucs(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbUCSTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_f64(10, item.origin.x));
        pairs.push(CodePair::new_f64(20, item.origin.y));
        pairs.push(CodePair::new_f64(30, item.origin.z));
        pairs.push(CodePair::new_f64(11, item.x_axis.x));
        pairs.push(CodePair::new_f64(21, item.x_axis.y));
        pairs.push(CodePair::new_f64(31, item.x_axis.z));
        pairs.push(CodePair::new_f64(12, item.y_axis.x));
        pairs.push(CodePair::new_f64(22, item.y_axis.y));
        pairs.push(CodePair::new_f64(32, item.y_axis.z));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(79, item.orthographic_view_type as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_f64(146, item.elevation));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_string(346, &item.__base_ucs_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(71, item.orthographic_type as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_f64(13, item.orthographic_origin.x));
            pairs.push(CodePair::new_f64(23, item.orthographic_origin.y));
            pairs.push(CodePair::new_f64(33, item.orthographic_origin.z));
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_views_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.views().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "VIEW"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.views() {
        pairs.push(CodePair::new_str(0, "VIEW"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::View(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbViewTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_f64(40, item.view_height));
        pairs.push(CodePair::new_f64(10, item.view_center_point.x));
        pairs.push(CodePair::new_f64(20, item.view_center_point.y));
        pairs.push(CodePair::new_f64(41, item.view_width));
        pairs.push(CodePair::new_f64(11, item.view_direction.x));
        pairs.push(CodePair::new_f64(21, item.view_direction.y));
        pairs.push(CodePair::new_f64(31, item.view_direction.z));
        pairs.push(CodePair::new_f64(12, item.target_point.x));
        pairs.push(CodePair::new_f64(22, item.target_point.y));
        pairs.push(CodePair::new_f64(32, item.target_point.z));
        pairs.push(CodePair::new_f64(42, item.lens_length));
        pairs.push(CodePair::new_f64(43, item.front_clipping_plane));
        pairs.push(CodePair::new_f64(44, item.back_clipping_plane));
        pairs.push(CodePair::new_f64(50, item.twist_angle));
        pairs.push(CodePair::new_i16(71, item.view_mode));
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(281, item.render_mode as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(72, as_i16(item.is_associated_ucs_present)));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(73, as_i16(item.is_camera_plottable)));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(332, &item.__background_object_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(334, &item.__selection_object_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(338, &item.__visual_style_object_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(361, &item.__sun_ownership_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_f64(110, item.ucs_origin.x));
            pairs.push(CodePair::new_f64(120, item.ucs_origin.y));
            pairs.push(CodePair::new_f64(130, item.ucs_origin.z));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_f64(111, item.ucs_x_axis.x));
            pairs.push(CodePair::new_f64(121, item.ucs_x_axis.y));
            pairs.push(CodePair::new_f64(131, item.ucs_x_axis.z));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_f64(112, item.ucs_y_axis.x));
            pairs.push(CodePair::new_f64(122, item.ucs_y_axis.y));
            pairs.push(CodePair::new_f64(132, item.ucs_y_axis.z));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_i16(79, item.orthographic_view_type as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_f64(146, item.ucs_elevation));
        }
        if drawing.header.version >= AcadVersion::R2007 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_string(345, &item.__ucs_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 && item.is_associated_ucs_present {
            pairs.push(CodePair::new_string(346, &item.__base_ucs_handle.as_string()));
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

#[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
fn add_view_ports_code_pairs(pairs: &mut Vec<CodePair>, drawing: &Drawing, write_handles: bool) {
    if !drawing.view_ports().any(|_| true) { // is empty
        return; // nothing to add
    }

    pairs.push(CodePair::new_str(0, "TABLE"));
    pairs.push(CodePair::new_str(2, "VPORT"));
    pairs.push(CodePair::new_str(100, "AcDbSymbolTable"));
    pairs.push(CodePair::new_i16(70, 0));
    for item in drawing.view_ports() {
        pairs.push(CodePair::new_str(0, "VPORT"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &DrawingItem::ViewPort(item).handle().as_string()));
        }

        if drawing.header.version >= AcadVersion::R14 {
            for group in &item.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }

        pairs.push(CodePair::new_str(100, "AcDbSymbolTableRecord"));
        pairs.push(CodePair::new_str(100, "AcDbViewportTableRecord"));
        pairs.push(CodePair::new_string(2, &item.name));
        pairs.push(CodePair::new_i16(70, 0));
        pairs.push(CodePair::new_f64(10, item.lower_left.x));
        pairs.push(CodePair::new_f64(20, item.lower_left.y));
        pairs.push(CodePair::new_f64(11, item.upper_right.x));
        pairs.push(CodePair::new_f64(21, item.upper_right.y));
        pairs.push(CodePair::new_f64(12, item.view_center.x));
        pairs.push(CodePair::new_f64(22, item.view_center.y));
        pairs.push(CodePair::new_f64(13, item.snap_base_point.x));
        pairs.push(CodePair::new_f64(23, item.snap_base_point.y));
        pairs.push(CodePair::new_f64(14, item.snap_spacing.x));
        pairs.push(CodePair::new_f64(24, item.snap_spacing.y));
        pairs.push(CodePair::new_f64(15, item.grid_spacing.x));
        pairs.push(CodePair::new_f64(25, item.grid_spacing.y));
        pairs.push(CodePair::new_f64(16, item.view_direction.x));
        pairs.push(CodePair::new_f64(26, item.view_direction.y));
        pairs.push(CodePair::new_f64(36, item.view_direction.z));
        pairs.push(CodePair::new_f64(17, item.target_view_point.x));
        pairs.push(CodePair::new_f64(27, item.target_view_point.y));
        pairs.push(CodePair::new_f64(37, item.target_view_point.z));
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_f64(40, item.view_height));
        }
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_f64(41, item.view_port_aspect_ratio));
        }
        pairs.push(CodePair::new_f64(42, item.lens_length));
        pairs.push(CodePair::new_f64(43, item.front_clipping_plane));
        pairs.push(CodePair::new_f64(44, item.back_clipping_plane));
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_f64(45, item.view_height));
        }
        pairs.push(CodePair::new_f64(50, item.snap_rotation_angle));
        pairs.push(CodePair::new_f64(51, item.view_twist_angle));
        pairs.push(CodePair::new_i16(71, item.view_mode.raw() as i16));
        pairs.push(CodePair::new_i16(72, item.circle_sides as i16));
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_i16(73, as_i16(item.fast_zoom)));
        }
        pairs.push(CodePair::new_i16(74, item.ucs_icon));
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_i16(75, as_i16(item.snap_on)));
        }
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_i16(76, as_i16(item.grid_on)));
        }
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_i16(77, item.snap_style as i16));
        }
        if drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_i16(78, item.snap_isometric_plane as i16));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(1, &item.plot_style_sheet));
        }
        if drawing.header.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_i16(281, item.render_mode as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 && drawing.header.version <= AcadVersion::R2004 {
            pairs.push(CodePair::new_i16(65, as_i16(item.has_own_ucs)));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_f64(110, item.ucs_origin.x));
            pairs.push(CodePair::new_f64(120, item.ucs_origin.y));
            pairs.push(CodePair::new_f64(130, item.ucs_origin.z));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_f64(111, item.ucs_x_axis.x));
            pairs.push(CodePair::new_f64(121, item.ucs_x_axis.y));
            pairs.push(CodePair::new_f64(131, item.ucs_x_axis.z));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_f64(112, item.ucs_y_axis.x));
            pairs.push(CodePair::new_f64(122, item.ucs_y_axis.y));
            pairs.push(CodePair::new_f64(132, item.ucs_y_axis.z));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_i16(79, item.orthographic_view_type as i16));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_f64(146, item.ucs_elevation));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_string(345, &item.__ucs_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2000 && item.has_own_ucs {
            pairs.push(CodePair::new_string(346, &item.__base_ucs_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(170, item.shade_plot_setting as i16));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(61, as_i16(item.major_grid_lines)));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(332, &item.__background_object_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(333, &item.__shade_plot_object_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_string(348, &item.__visual_style_object_handle.as_string()));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_bool(292, item.is_default_lighting_on));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_i16(282, item.default_lighting_type as i16));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_f64(141, item.brightness));
        }
        if drawing.header.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_f64(142, item.contrast));
        }
        if drawing.header.version >= AcadVersion::R2007 && item.ambient_color.raw_value() != 7 {
            pairs.push(CodePair::new_i16(62, Color::raw_value(&item.ambient_color)));
        }
        if drawing.header.version >= AcadVersion::R2007 && item.ambient_color_i32 != 0 {
            pairs.push(CodePair::new_i32(421, item.ambient_color_i32));
        }
        if drawing.header.version >= AcadVersion::R2007 && item.ambient_color_name != "BLACK" {
            pairs.push(CodePair::new_string(431, &item.ambient_color_name));
        }
        for x in &item.x_data {
            x.add_code_pairs(pairs, drawing.header.version);
        }
    }

    pairs.push(CodePair::new_str(0, "ENDTAB"));
}

