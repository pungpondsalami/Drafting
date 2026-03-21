// The contents of this file are automatically generated and should not be modified directly.  See the `build` directory.

// types from `lib.rs`.
use crate::{
    CodePair,
    Color,
    DxfError,
    DxfResult,
    Handle,
    LineWeight,
    Point,
    Vector,
};
use crate::helper_functions::*;

use crate::enums::*;
use enum_primitive::FromPrimitive;

use std::time::Duration;

use chrono::{DateTime, Local, Utc};

use uuid::Uuid;
/// Contains common properties for the DXF file.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Header {
    /// The $ACADVER header variable.  The AutoCAD drawing database version number.
    pub version: AcadVersion,
    /// The $ACADMAINTVER header variable.  Maintenance version number (should be ignored).  Minimum AutoCAD version: R14.
    pub maintenance_version: i16,
    /// The $DWGCODEPAGE header variable.  Drawing code page; set to the system code page when a new drawing is created, but not otherwise maintained by AutoCAD.  Minimum AutoCAD version: R13.
    pub drawing_code_page: String,
    /// The $LASTSAVEDBY header variable.  Name of the last user to modify the file.  Minimum AutoCAD version: R2004.
    pub last_saved_by: String,
    /// The $REQUIREDVERSIONS header variable.  Unknown.  Minimum AutoCAD version: R2013.
    pub required_versions: i64,
    /// The $INSBASE header variable.  Insertion base set by BASE command (in WCS).
    pub insertion_base: Point,
    /// The $EXTMIN header variable.  X, Y, and Z drawing extents lower-left corner (in WCS).
    pub minimum_drawing_extents: Point,
    /// The $EXTMAX header variable.  X, Y, and Z drawing extents upper-right corner (in WCS).
    pub maximum_drawing_extents: Point,
    /// The $LIMMIN header variable.  XY drawing limits lower-left corner (in WCS).
    pub minimum_drawing_limits: Point,
    /// The $LIMMAX header variable.  XY drawing limits upper-right corner (in WCS).
    pub maximum_drawing_limits: Point,
    /// The $ORTHOMODE header variable.  Ortho mode on.
    pub draw_orthogonal_lines: bool,
    /// The $REGENMODE header variable.  REGENAUTO mode on.
    pub use_regen_mode: bool,
    /// The $FILLMODE header variable.  Fill mode on.
    pub fill_mode_on: bool,
    /// The $QTEXTMODE header variable.  Quick text mode on.
    pub use_quick_text_mode: bool,
    /// The $MIRRTEXT header variable.  Mirror text.
    pub mirror_text: bool,
    /// The $DRAGMODE header variable.  Controls the way dragged objects are displayed.  Maximum AutoCAD version: R14.
    pub drag_mode: DragMode,
    /// The $LTSCALE header variable.  Global line type scale.
    pub line_type_scale: f64,
    /// The $OSMODE header variable.  Running object snap modes.  Maximum AutoCAD version: R14.
    pub object_snap_flags: i32,
    /// The $ATTMODE header variable.  Attribute visibility.
    pub attribute_visibility: AttributeVisibility,
    /// The $TEXTSIZE header variable.  Default text height.
    pub default_text_height: f64,
    /// The $TRACEWID header variable.  Default trace width.
    pub trace_width: f64,
    /// The $TEXTSTYLE header variable.  Current text style name.
    pub text_style: String,
    /// The $CLAYER header variable.  Current layer name.
    pub current_layer: String,
    /// The $CELTYPE header variable.  Entity line type name, or BYBLOCK or BYLAYER.
    pub current_entity_line_type: String,
    /// The $CECOLOR header variable.  Current entity color.
    pub current_entity_color: Color,
    /// The $CELTSCALE header variable.  Current entity line type scale.  Minimum AutoCAD version: R13.
    pub current_entity_line_type_scale: f64,
    /// The $DELOBJ header variable.  Controls object deletion.  Minimum AutoCAD version: R13.  Maximum AutoCAD version: R14.
    pub retain_deleted_objects: bool,
    /// The $DISPSILH header variable.  Controls the display of silhouette curves of body objects in wireframe mode.  Minimum AutoCAD version: R13.
    pub display_silhouette_curves_in_wireframe_mode: bool,
    /// The $DRAGVS header variable.  Hard-pointer to visual style when creating 3D solid primitives.  Minimum AutoCAD version: R2007.
    pub solid_visual_style_pointer: Handle,
    /// The $DIMSCALE header variable.  Overall dimensioning scale factor.
    pub dimensioning_scale_factor: f64,
    /// The $DIMASZ header variable.  Dimensioning arrow size.
    pub dimensioning_arrow_size: f64,
    /// The $DIMEXO header variable.  Extension line offset.
    pub dimension_extension_line_offset: f64,
    /// The $DIMDLI header variable.  Dimension line increment.
    pub dimension_line_increment: f64,
    /// The $DIMRND header variable.  Rounding value for dimension distances.
    pub dimension_distance_rounding_value: f64,
    /// The $DIMDLE header variable.  Dimension line extension.
    pub dimension_line_extension: f64,
    /// The $DIMEXE header variable.  Extension line extension.
    pub dimension_extension_line_extension: f64,
    /// The $DIMTP header variable.  Plus tolerance.
    pub dimension_plus_tolerance: f64,
    /// The $DIMTM header variable.  Minus tolerance.
    pub dimension_minus_tolerance: f64,
    /// The $DIMTXT header variable.  Dimensioning text height.
    pub dimensioning_text_height: f64,
    /// The $DIMCEN header variable.  Size of center mark/lines.
    pub center_mark_size: f64,
    /// The $DIMTSZ header variable.  Dimensioning tick size.
    pub dimensioning_tick_size: f64,
    /// The $DIMTOL header variable.  Dimension tolerances generated.
    pub generate_dimension_tolerances: bool,
    /// The $DIMLIM header variable.  Dimension limits generated.
    pub generate_dimension_limits: bool,
    /// The $DIMTIH header variable.  Text inside horizontal.
    pub dimension_text_inside_horizontal: bool,
    /// The $DIMTOH header variable.  Text outside horizontal.
    pub dimension_text_outside_horizontal: bool,
    /// The $DIMSE1 header variable.  Suppression of first extension line.  Minimum AutoCAD version: R12.
    pub suppress_first_dimension_extension_line: bool,
    /// The $DIMSE2 header variable.  Suppression of second extension line.  Minimum AutoCAD version: R12.
    pub suppress_second_dimension_extension_line: bool,
    /// The $DIMTAD header variable.  Text above dimension line.
    pub text_above_dimension_line: bool,
    /// The $DIMZIN header variable.  Controls suppression of zeros for primary unit values.
    pub dimension_unit_zero_suppression: UnitZeroSuppression,
    /// The $DIMBLK header variable.  Arrow block name.
    pub arrow_block_name: String,
    /// The $DIMASO header variable.  Controls associative dimensioning.
    pub create_associative_dimensioning: bool,
    /// The $DIMSHO header variable.  Recompute dimensions while dragging.
    pub recompute_dimensions_while_dragging: bool,
    /// The $DIMPOST header variable.  General dimensioning suffix.
    pub dimensioning_suffix: String,
    /// The $DIMAPOST header variable.  Alternate dimensioning suffix.
    pub alternate_dimensioning_suffix: String,
    /// The $DIMALT header variable.  Alternate unit dimensioning performed.
    pub use_alternate_dimensioning: bool,
    /// The $DIMALTD header variable.  Alternate unit decimal places.
    pub alternate_dimensioning_decimal_places: i16,
    /// The $DIMALTF header variable.  Alternate unit scale factor.
    pub alternate_dimensioning_scale_factor: f64,
    /// The $DIMLFAC header variable.  Linear measurements scale factor.
    pub dimension_linear_measurements_scale_factor: f64,
    /// The $DIMTOFL header variable.  If text is outside extensions, force line extensions between extensions.
    pub force_dimension_line_extensions_outside_if_text_is: bool,
    /// The $DIMTVP header variable.  Text vertical position.
    pub dimension_vertical_text_position: f64,
    /// The $DIMTIX header variable.  Force text inside extensions.
    pub force_dimension_text_inside_extensions: bool,
    /// The $DIMSOXD header variable.  Suppress outside-extensions dimension lines.
    pub suppress_outside_extension_dimension_lines: bool,
    /// The $DIMSAH header variable.  Use separate arrow blocks.
    pub use_separate_arrow_blocks_for_dimensions: bool,
    /// The $DIMBLK1 header variable.  First arrow block name.
    pub first_arrow_block_name: String,
    /// The $DIMBLK2 header variable.  Second arrow block name.
    pub second_arrow_block_name: String,
    /// The $DIMSTYLE header variable.  Dimension style name.
    pub dimension_style_name: String,
    /// The $DIMCLRD header variable.  Dimension line color.  Minimum AutoCAD version: R11.
    pub dimension_line_color: Color,
    /// The $DIMCLRE header variable.  Dimension extension line color.  Minimum AutoCAD version: R11.
    pub dimension_extension_line_color: Color,
    /// The $DIMCLRT header variable.  Dimension text color.  Minimum AutoCAD version: R11.
    pub dimension_text_color: Color,
    /// The $DIMTFAC header variable.  Dimension tolerance display factor.  Minimum AutoCAD version: R12.
    pub dimension_tolerance_display_scale_factor: f64,
    /// The $DIMGAP header variable.  Dimension line gap.  Minimum AutoCAD version: R11.
    pub dimension_line_gap: f64,
    /// The $DIMJUST header variable.  Horizontal dimension text position.  Minimum AutoCAD version: R13.
    pub dimension_text_justification: DimensionTextJustification,
    /// The $DIMTOLJ header variable.  Vertical justification for tolerance values.  Minimum AutoCAD version: R13.
    pub dimension_tolerance_vertical_justification: Justification,
    /// The $DIMTZIN header variable.  Controls suppression of zeros for tolerance values.  Minimum AutoCAD version: R13.
    pub dimension_tolerance_zero_suppression: UnitZeroSuppression,
    /// The $DIMALTZ header variable.  Controls suppression of zeros for alternate unit dimension values.  Minimum AutoCAD version: R13.
    pub alternate_dimensioning_zero_supression: UnitZeroSuppression,
    /// The $DIMALTTZ header variable.  Controls suppression of zeros for alternate tolerance values.  Minimum AutoCAD version: R13.
    pub alternate_dimensioning_tolerance_zero_supression: UnitZeroSuppression,
    /// The $DIMFIT header variable.  Placement of text and arrowheads.  Minimum AutoCAD version: R13.  Maximum AutoCAD version: R14.
    pub dimension_text_and_arrow_placement: DimensionFit,
    /// The $DIMUPT header variable.  Cursor functionality for user-positioned text.  Minimum AutoCAD version: R13.
    pub dimension_cursor_controls_text_position: bool,
    /// The $DIMUNIT header variable.  Units format for all dimension style family members except angular.  Minimum AutoCAD version: R13.  Maximum AutoCAD version: R14.
    pub dimension_unit_format: UnitFormat,
    /// The $DIMDEC header variable.  Number of decimal places for the tolerance values of a primary units dimension.  Minimum AutoCAD version: R13.
    pub dimension_unit_tolerance_decimal_places: i16,
    /// The $DIMTDEC header variable.  Number of decimal places to display the tolerance values.  Minimum AutoCAD version: R13.
    pub dimension_tolerance_decimal_places: i16,
    /// The $DIMALTU header variable.  Units format for alternate units of all dimension style family members except angular.  Minimum AutoCAD version: R13.
    pub alternate_dimensioning_units: UnitFormat,
    /// The $DIMALTTD header variable.  Number of decimal places for tolerance values of an alternate units dimension.  Minimum AutoCAD version: R13.
    pub alternate_dimensioning_tolerance_decimal_places: i16,
    /// The $DIMTXSTY header variable.  Dimension text style.  Minimum AutoCAD version: R13.
    pub dimension_text_style: String,
    /// The $DIMAUNIT header variable.  Angle format for angular dimensions.  Minimum AutoCAD version: R13.
    pub dimensioning_angle_format: AngleFormat,
    /// The $DIMADEC header variable.  Number of precision places displayed in angular dimensions.  Minimum AutoCAD version: R2000.
    pub angular_dimension_precision: i16,
    /// The $DIMALTRND header variable.  Determines rounding of alternate units.  Minimum AutoCAD version: R2000.
    pub alternate_dimensioning_unit_rounding: f64,
    /// The $DIMAZIN header variable.  Controls suppression of zeros for angular dimensions.  Minimum AutoCAD version: R2000.
    pub dimension_angle_zero_suppression: UnitZeroSuppression,
    /// The $DIMDSEP header variable.  Single-character decimal separator used when creating dimensions whose unit format is decimal.  Minimum AutoCAD version: R2000.
    pub dimension_decimal_separator_char: char,
    /// The $DIMFRAC header variable.  Sets the fraction format when DIMLUNIT is set to Architectural or Fractional.  Minimum AutoCAD version: R2000.
    pub dimension_text_height_scale_factor: DimensionFractionFormat,
    /// The $DIMLDRBLK header variable.  Arrow block name for leaders.  Minimum AutoCAD version: R2000.
    pub dimension_leader_block_name: String,
    /// The $DIMLUNIT header variable.  Sets units for all dimension types except angular.  Minimum AutoCAD version: R2000.
    pub dimension_non_angular_units: NonAngularUnits,
    /// The $DIMLWD header variable.  Dimension line lineweight.  Minimum AutoCAD version: R2000.
    pub dimension_line_weight: LineWeight,
    /// The $DIMLWE header variable.  Extension line lineweight.  Minimum AutoCAD version: R2000.
    pub dimension_extension_line_weight: LineWeight,
    /// The $DIMTMOVE header variable.  Dimension text movement rules.  Minimum AutoCAD version: R2000.
    pub dimension_text_movement_rule: DimensionTextMovementRule,
    /// The $DIMFXL header variable.  Sets the total length of the extension lines starting from the dimension line toward the dimension origin.  Minimum AutoCAD version: R2007.
    pub dimension_line_fixed_length: f64,
    /// The $DIMFXLON header variable.  Controls whether extension lines are set to a fixed length.  Minimum AutoCAD version: R2007.
    pub dimension_line_fixed_length_on: bool,
    /// The $DIMJOGANG header variable.  Determines the angle of the transverse segment of the dimension line in a jogged radius dimension.  Minimum AutoCAD version: R2007.
    pub dimension_transverse_segment_angle_in_jogged_radius: f64,
    /// The $DIMTFILL header variable.  Controls the background of dimension text.  Minimum AutoCAD version: R2007.
    pub dimension_text_background_color_mode: DimensionTextBackgroundColorMode,
    /// The $DIMTFILLCLR header variable.  Sets the color for the text background in dimensions.  Minimum AutoCAD version: R2007.
    pub dxf_dimension_text_background_custom_color: Color,
    /// The $DIMARCSYM header variable.  Controls the display of the arc symbol in an arc length dimension.  Minimum AutoCAD version: R2007.
    pub dimension_arc_symbol_display_mode: DimensionArcSymbolDisplayMode,
    /// The $DIMLTYPE header variable.  Sets the line type of the dimension line.  Minimum AutoCAD version: R2007.
    pub dimension_line_type: String,
    /// The $DIMLTEX1 header variable.  Sets the line type of the first extension line.  Minimum AutoCAD version: R2007.
    pub dimension_first_extension_line_type: String,
    /// The $DIMLTEX2 header variable.  Sets the line type of the second extension line.  Minimum AutoCAD version: R2007.
    pub dimension_second_extension_line_type: String,
    /// The $DIMTXTDIRECTION header variable.  Specifies the reading direction of the dimension text.  Minimum AutoCAD version: R2010.
    pub dimension_text_direction: TextDirection,
    /// The $LUNITS header variable.  Units format for coordinates and distances.
    pub unit_format: UnitFormat,
    /// The $LUPREC header variable.  Units precision for coordinates and distances.
    pub unit_precision: i16,
    /// The $SKETCHINC header variable.  Sketch record increment.
    pub sketch_record_increment: f64,
    /// The $FILLETRAD header variable.  Fillet radius.
    pub fillet_radius: f64,
    /// The $AUNITS header variable.  Units format for angles.
    pub angle_unit_format: AngleFormat,
    /// The $AUPREC header variable.  Units precision for angles.
    pub angle_unit_precision: i16,
    /// The $MENU header variable.  Name of menu file.
    pub file_name: String,
    /// The $ELEVATION header variable.  Current elevation set by ELEV command.
    pub elevation: f64,
    /// The $PELEVATION header variable.  Current paper space elevation.  Minimum AutoCAD version: R11.
    pub paperspace_elevation: f64,
    /// The $THICKNESS header variable.  Current thickness set by ELEV command.
    pub thickness: f64,
    /// The $LIMCHECK header variable.  Limits checking.
    pub use_limits_checking: bool,
    /// The $BLIPMODE header variable.  Display blips for click locations.  Maximum AutoCAD version: R14.
    pub blip_mode: bool,
    /// The $CHAMFERA header variable.  First chamfer distance.
    pub first_chamfer_distance: f64,
    /// The $CHAMFERB header variable.  Second chamfer distance.
    pub second_chamfer_distance: f64,
    /// The $CHAMFERC header variable.  Chamfer length.  Minimum AutoCAD version: R14.
    pub chamfer_length: f64,
    /// The $CHAMFERD header variable.  Chamfer angle.  Minimum AutoCAD version: R14.
    pub chamfer_angle: f64,
    /// The $SKPOLY header variable.  Controls polyline sketch mode.
    pub polyline_sketch_mode: PolySketchMode,
    /// The $TDCREATE header variable.  Local date/time of drawing creation.
    pub creation_date: DateTime<Local>,
    /// The $TDUCREATE header variable.  Universal date/time the drawing was created.  Minimum AutoCAD version: R2000.
    pub creation_date_universal: DateTime<Utc>,
    /// The $TDUPDATE header variable.  Local date/time of last drawing update.
    pub update_date: DateTime<Local>,
    /// The $TDUUPDATE header variable.  Universal date/time of the last update/save.  Minimum AutoCAD version: R2000.
    pub update_date_universal: DateTime<Utc>,
    /// The $TDINDWG header variable.  Cumulative editing time for this drawing.
    pub time_in_drawing: Duration,
    /// The $TDUSRTIMER header variable.  User-elapsed timer.
    pub user_elapsed_timer: Duration,
    /// The $USRTIMER header variable.  User timer on.
    pub user_timer_on: bool,
    /// The $ANGBASE header variable.  Angle 0 direction.
    pub angle_zero_direction: f64,
    /// The $ANGDIR header variable.  Angle directions.
    pub angle_direction: AngleDirection,
    /// The $PDMODE header variable.  Point display mode.
    pub point_display_mode: i32,
    /// The $PDSIZE header variable.  Point display size.
    pub point_display_size: f64,
    /// The $PLINEWID header variable.  Default polyline width.
    pub default_polyline_width: f64,
    /// The $COORDS header variable.  Controls the display of coordinates.  Maximum AutoCAD version: R14.
    pub coordinate_display: CoordinateDisplay,
    /// The $SPLFRAME header variable.  Controls the display of helixes and smoothed mesh objects.  Maximum AutoCAD version: R2013.
    pub display_spline_polygon_control: bool,
    /// The $SPLINETYPE header variable.  Spline curve type for PEDIT Spline.
    pub pedit_spline_curve_type: PolylineCurvedAndSmoothSurfaceType,
    /// The $SPLINESEGS header variable.  Number of line segments per spline hatch.
    pub line_segments_per_spline_patch: i16,
    /// The $ATTDIA header variable.  Controls whether the INSERT command uses a dialog box for attribute value entry.  Maximum AutoCAD version: R14.
    pub show_attribute_entry_dialogs: bool,
    /// The $ATTREQ header variable.  Controls whether INSERT uses default attribute settings during insertion of blocks.  Maximum AutoCAD version: R14.
    pub prompt_for_attribute_on_insert: bool,
    /// The $HANDLING header variable.  Handles available.  Maximum AutoCAD version: R12.
    pub handles_enabled: bool,
    /// The $HANDSEED header variable.  Next available handle.
    pub next_available_handle: Handle,
    /// The $SURFTAB1 header variable.  Number of mesh tabulations in first direction.
    pub mesh_tabulations_in_first_direction: i16,
    /// The $SURFTAB2 header variable.  Number of mesh tabulations in second direction.
    pub mesh_tabulations_in_second_direction: i16,
    /// The $SURFTYPE header variable.  Surface type for PEDIT Smooth.
    pub pedit_smooth_surface_type: PolylineCurvedAndSmoothSurfaceType,
    /// The $SURFU header variable.  Surface density (for PEDIT Smooth) in M direction.
    pub pedit_smooth_m_densith: i16,
    /// The $SURFV header variable.  Surface density (for PEDIT Smooth) in N direction.
    pub pedit_smooth_n_densith: i16,
    /// The $UCSBASE header variable.  Name of the UCS that defines the origin and orientation of orthographic UCS settings.  Minimum AutoCAD version: R2000.
    pub ucs_definition_name: String,
    /// The $UCSNAME header variable.  Name of current UCS.
    pub ucs_name: String,
    /// The $UCSORG header variable.  Origin of current UCS (in WCS).
    pub ucs_origin: Point,
    /// The $UCSXDIR header variable.  Direction of the current UCS X axis (in WCS).
    pub ucs_x_axis: Vector,
    /// The $UCSYDIR header variable.  Direction of the current UCS Y axis (in WCS).
    pub ucs_y_axis: Vector,
    /// The $UCSORTHOREF header variable.  If model space UCS is orthographic (UCSORTHOVIEW not equal to 0), this is the name of the UCS that the orthographic UCS is relative to. If blank, UCS is relative to WORLD.  Minimum AutoCAD version: R2000.
    pub ortho_ucs_reference: String,
    /// The $UCSORTHOVIEW header variable.  Orthographic view type of model space UCS.  Minimum AutoCAD version: R2000.
    pub orthgraphic_view_type: OrthographicViewType,
    /// The $UCSORGTOP header variable.  Point which becomes the new UCS origin after changing model space UCS to TOP when UCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub ucs_origin_top: Point,
    /// The $UCSORGBOTTOM header variable.  Point which becomes the new UCS origin after changing model space UCS to BOTTOM when UCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub ucs_origin_bottom: Point,
    /// The $UCSORGLEFT header variable.  Point which becomes the new UCS origin after changing model space UCS to LEFT when UCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub ucs_origin_left: Point,
    /// The $UCSORGRIGHT header variable.  Point which becomes the new UCS origin after changing model space UCS to RIGHT when UCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub ucs_origin_right: Point,
    /// The $UCSORGFRONT header variable.  Point which becomes the new UCS origin after changing model space UCS to FRONT when UCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub ucs_origin_front: Point,
    /// The $UCSORGBACK header variable.  Point which becomes the new UCS origin after changing model space UCS to BACK when UCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub ucs_origin_back: Point,
    /// The $PUCSBASE header variable.  Name of the UCS that defines the origin and orientation of orthographics UCS settings (paper space only).  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_definition_name: String,
    /// The $PUCSNAME header variable.  Current paper space UCS name.  Minimum AutoCAD version: R11.
    pub paperspace_ucs_name: String,
    /// The $PUCSORG header variable.  Current paper space UCS origin.  Minimum AutoCAD version: R11.
    pub paperspace_ucs_origin: Point,
    /// The $PUCSXDIR header variable.  Current paper space UCS X axis.  Minimum AutoCAD version: R11.
    pub paperspace_x_axis: Vector,
    /// The $PUCSYDIR header variable.  Current paper space UCS Y axis.  Minimum AutoCAD version: R11.
    pub paperspace_y_axis: Vector,
    /// The $PUCSORTHOREF header variable.  If paper space UCS is orthographic (PUCSORTHOVIEW not equal to 0), this is the name of the UCS that the orthographic UCS is relative to. If blank, UCS is relative to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ortho_ucs_reference: String,
    /// The $PUCSORTHOVIEW header variable.  Orthographic view type of paper space UCS.  Minimum AutoCAD version: R2000.
    pub paperspace_orthographic_view_type: OrthographicViewType,
    /// The $PUCSORGTOP header variable.  Point which becomes the new UCS origin after changing paper space UCS to TOP when PUCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_origin_top: Point,
    /// The $PUCSORGBOTTOM header variable.  Point which becomes the new UCS origin after changing paper space UCS to BOTTOM when PUCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_origin_bottom: Point,
    /// The $PUCSORGLEFT header variable.  Point which becomes the new UCS origin after changing paper space UCS to LEFT when PUCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_origin_left: Point,
    /// The $PUCSORGRIGHT header variable.  Point which becomes the new UCS origin after changing paper space UCS to RIGHT when PUCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_origin_right: Point,
    /// The $PUCSORGFRONT header variable.  Point which becomes the new UCS origin after changing paper space UCS to FRONT when PUCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_origin_front: Point,
    /// The $PUCSORGBACK header variable.  Point which becomes the new UCS origin after changing paper space UCS to BACK when PUCSBASE is set to WORLD.  Minimum AutoCAD version: R2000.
    pub paperspace_ucs_origin_back: Point,
    /// The $USERI1 header variable.  Integer variable intended for use by third-party developers.
    pub user_int1: i16,
    /// The $USERI2 header variable.  Integer variable intended for use by third-party developers.
    pub user_int2: i16,
    /// The $USERI3 header variable.  Integer variable intended for use by third-party developers.
    pub user_int3: i16,
    /// The $USERI4 header variable.  Integer variable intended for use by third-party developers.
    pub user_int4: i16,
    /// The $USERI5 header variable.  Integer variable intended for use by third-party developers.
    pub user_int5: i16,
    /// The $USERR1 header variable.  Real variable indented for use by third-party developers.
    pub user_real1: f64,
    /// The $USERR2 header variable.  Real variable indented for use by third-party developers.
    pub user_real2: f64,
    /// The $USERR3 header variable.  Real variable indented for use by third-party developers.
    pub user_real3: f64,
    /// The $USERR4 header variable.  Real variable indented for use by third-party developers.
    pub user_real4: f64,
    /// The $USERR5 header variable.  Real variable indented for use by third-party developers.
    pub user_real5: f64,
    /// The $WORLDVIEW header variable.  Set UCS to WCS during DVIEW/VPOINT.
    pub set_ucs_to_wcs_in_d_view_or_v_point: bool,
    /// The $SHADEDGE header variable.  Controls shading of faces.  Minimum AutoCAD version: R11.
    pub edge_shading: ShadeEdgeMode,
    /// The $SHADEDIF header variable.  Percent ambient/diffuse light; range 1-100.  Minimum AutoCAD version: R11.
    pub percent_ambient_to_diffuse: i16,
    /// The $TILEMODE header variable.  Use previous release compatibility mode.  Minimum AutoCAD version: R11.
    pub previous_release_tile_compatability: bool,
    /// The $MAXACTVP header variable.  Sets the maximum number of viewports to be regenerated.  Minimum AutoCAD version: R11.
    pub maximum_active_viewports: i16,
    /// The $PINSBASE header variable.  Paper space insertion base point.  Minimum AutoCAD version: R14.
    pub paperspace_insertion_base: Point,
    /// The $PLIMCHECK header variable.  Limits checking in paper space.  Minimum AutoCAD version: R11.
    pub limit_checking_in_paperspace: bool,
    /// The $PEXTMIN header variable.  Minimum X, Y, and Z extents for paper space.  Minimum AutoCAD version: R11.
    pub paperspace_minimum_drawing_extents: Point,
    /// The $PEXTMAX header variable.  Maximum X, Y, and Z extents for paper space.  Minimum AutoCAD version: R11.
    pub paperspace_maximum_drawing_extents: Point,
    /// The $PLIMMIN header variable.  Minimum X and Y limits in paper space.  Minimum AutoCAD version: R11.
    pub paperspace_minimum_drawing_limits: Point,
    /// The $PLIMMAX header variable.  Maximum X and Y limits in paper space.  Minimum AutoCAD version: R11.
    pub paperspace_maximum_drawing_limits: Point,
    /// The $UNITMODE header variable.  Display fractions, feet-and-inches, and surveyor's angles in input format.  Minimum AutoCAD version: R11.
    pub display_fractions_in_input: bool,
    /// The $VISRETAIN header variable.  Retain xref-dependent visibility settings.  Minimum AutoCAD version: R12.
    pub retain_x_ref_dependent_visibility_settings: bool,
    /// The $PLINEGEN header variable.  Governs the generation of line type patterns around the vertices of a 2D polyline.  Minimum AutoCAD version: R11.
    pub is_polyline_continuous_around_verticies: bool,
    /// The $PSLTSCALE header variable.  Controls paper space line type scaling.  Minimum AutoCAD version: R11.
    pub scale_line_types_in_paperspace: bool,
    /// The $TREEDEPTH header variable.  Specifies the maximum depth of the spatial index.  Minimum AutoCAD version: R14.
    pub spacial_index_max_depth: i16,
    /// The $PICKSTYLE header variable.  Controls the group selection and associative hatch selection.  Minimum AutoCAD version: R13.  Maximum AutoCAD version: R14.
    pub pick_style: PickStyle,
    /// The $CMLSTYLE header variable.  Current multiline style name.  Minimum AutoCAD version: R13.  Maximum AutoCAD version: R13.
    pub current_multiline_style: String,
    /// The $CMLJUST header variable.  Current multiline justification.  Minimum AutoCAD version: R13.
    pub current_multiline_justification: Justification,
    /// The $CMLSCALE header variable.  Current multiline scale.  Minimum AutoCAD version: R13.
    pub current_multiline_scale: f64,
    /// The $PROXYGRAPHICS header variable.  Controls the saving of proxy object images.  Minimum AutoCAD version: R14.
    pub save_proxy_graphics: bool,
    /// The $MEASUREMENT header variable.  Sets drawing units.  Minimum AutoCAD version: R14.
    pub drawing_units: DrawingUnits,
    /// The $CELWEIGHT header variable.  Lineweight of new objects.  Minimum AutoCAD version: R2000.
    pub new_object_line_weight: LineWeight,
    /// The $ENDCAPS header variable.  Lineweight endcaps setting for new objects.  Minimum AutoCAD version: R2000.
    pub end_cap_setting: EndCapSetting,
    /// The $JOINSTYLE header variable.  Lineweight join setting for new objects.  Minimum AutoCAD version: R2000.
    pub lineweight_joint_setting: JoinStyle,
    /// The $LWDISPLAY header variable.  Controls the display of lineweights on the Model or Layout tab.  Minimum AutoCAD version: R2000.
    pub display_linewieght_in_model_and_layout_tab: bool,
    /// The $INSUNITS header variable.  Default drawing units for AutoCAD DesignCenter blocks.  Minimum AutoCAD version: R2000.
    pub default_drawing_units: Units,
    /// The $HYPERLINKBASE header variable.  Path for all relative hyperlinks in the drawing.  If null, the drawing path is used.  Minimum AutoCAD version: R2000.
    pub hyperlink_base: String,
    /// The $STYLESHEET header variable.  Path to the stylesheet for the drawing.  Minimum AutoCAD version: R2000.
    pub stylesheet: String,
    /// The $XEDIT header variable.  Controls whether the current drawing can be edited in-place when being referenced by another drawing.  Minimum AutoCAD version: R2000.
    pub can_use_in_place_reference_editing: bool,
    /// The $CEPSNID header variable.  PlotStyle handle of new objects.  Minimum AutoCAD version: R2000.
    pub new_object_plot_style_handle: Handle,
    /// The $CEPSNTYPE header variable.  Plot style of new objects.  Minimum AutoCAD version: R2000.
    pub new_object_plot_style: PlotStyle,
    /// The $PSTYLEMODE header variable.  Indicates whether the current drawing is in a Color-Dependent or Named Plot Style mode.  Minimum AutoCAD version: R2000.
    pub uses_color_dependent_plot_style_tables: bool,
    /// The $FINGERPRINTGUID header variable.  Set at creation time, uniquely identifies a particular drawing.  Minimum AutoCAD version: R2000.
    pub fingerprint_guid: Uuid,
    /// The $VERSIONGUID header variable.  Uniquely identifies a particular version of a drawing.  Updated when the drawing is modified.  Minimum AutoCAD version: R2000.
    pub version_guid: Uuid,
    /// The $EXTNAMES header variable.  Controls symbol table naming.  Minimum AutoCAD version: R2000.
    pub use_acad2000_symbol_table_naming: bool,
    /// The $PSVPSCALE header variable.  View scale factor for new viewports.  Minimum AutoCAD version: R2000.
    pub viewport_view_scale_factor: f64,
    /// The $OLESTARTUP header variable.  Controls whether the source application of an embedded OLE object loads when plotting.  Minimum AutoCAD version: R2000.
    pub ole_startup: bool,
    /// The $SORTENTS header variable.  Controls the object sorting methods; accessible from the Options dialog box User Preferences tab.  Minimum AutoCAD version: R2004.
    pub object_sorting_methods_flags: i32,
    /// The $INDEXCTL header variable.  Controls whether layer and spatial indexes are created and saved in drawing files.  Minimum AutoCAD version: R2004.
    pub layer_and_spatial_index_save_mode: LayerAndSpatialIndexSaveMode,
    /// The $HIDETEXT header variable.  Ignore text objects.  Minimum AutoCAD version: R2004.
    pub hide_text_objects_when_producint_hidden_view: bool,
    /// The $XCLIPFRAME header variable.  Controls the visibility of xref clipping boundaries.  Minimum AutoCAD version: R2004.  Maximum AutoCAD version: R2007.
    pub is_x_ref_clipping_boundary_visible: XrefClippingBoundaryVisibility,
    /// The $HALOGAP header variable.  Specifies a gap to be displayed where an object is hidden by another object; the value is specified as a percent of one unit and is independent of the zoom level.  A haloed line is shortened at the point where it is hidden when HIDE or the Hidden option of SHADEMODE is used.  Minimum AutoCAD version: R2004.
    pub halo_gap_percent: f64,
    /// The $OBSCOLOR header variable.  Specifies the color of obscured lines.  An obscured line is a hidden line made visible by changing its color and line type and is visible only when the HIDE or SHADEMODE command is used.  The OBSCUREDCOLOR setting is visible only if the OBSCUREDLTYPE is turned ON by setting it to a value other than 0.  Minimum AutoCAD version: R2004.
    pub obscured_line_color: Color,
    /// The $OBSLTYPE header variable.  Specifies the line type of obscured lines.  Obscured line types are independent of zoom level, unlike regular AutoCAD line types.  Value 0 turns off display of obscured lines and is the default.  Minimum AutoCAD version: R2004.
    pub obscured_line_type_style: LineTypeStyle,
    /// The $INTERSECTIONDISPLAY header variable.  Specifies the display of intersection polylines.  Minimum AutoCAD version: R2004.
    pub display_intersection_polylines: bool,
    /// The $INTERSECTIONCOLOR header variable.  Specifies the entity color of intersection polylines.  Minimum AutoCAD version: R2004.
    pub intersection_polyline_color: Color,
    /// The $DIMASSOC header variable.  Controls the associativity of dimension objects.  Minimum AutoCAD version: R2004.
    pub dimension_object_associativity: DimensionAssociativity,
    /// The $PROJECTNAME header variable.  Assigns a project name to the current drawing.  Used when an external reference or image is not found on its original path.  The project name points to a section in the registry that can contain one or more search paths for each project name defined.  Project names and their search directories are created from the Files tab of the Options dialog box.  Minimum AutoCAD version: R2004.
    pub project_name: String,
    /// The $CAMERADISPLAY header variable.  Turns the display of camera objects on or off.  Minimum AutoCAD version: R2007.
    pub use_camera_display: bool,
    /// The $LENSLENGTH header variable.  Stores the length of the lens in millimeters used in perspective viewing.  Minimum AutoCAD version: R2007.
    pub lens_length: f64,
    /// The $CAMERAHEIGHT header variable.  Specifies the default height for new camera objects.  Minimum AutoCAD version: R2007.
    pub camera_height: f64,
    /// The $STEPSPERSEC header variable.  Specifies the number of steps taken per second when you are in walk or fly mode.  Minimum AutoCAD version: R2007.
    pub steps_per_second_in_walk_or_fly_mode: f64,
    /// The $STEPSIZE header variable.  Specifies the size of each step when in walk or fly mode, in drawing units.  Minimum AutoCAD version: R2007.
    pub step_size_in_walk_or_fly_mode: f64,
    /// The $3DDWFPREC header variable.  Controls the precision of 3D DWF or 3D DWFx publishing.  Minimum AutoCAD version: R2007.
    pub dwf_3d_precision: Dwf3DPrecision,
    /// The $PSOLWIDTH header variable.  Controls the default width for a swept solid object created with the POLYSOLID command.  Minimum AutoCAD version: R2007.
    pub last_poly_solid_width: f64,
    /// The $PSOLHEIGHT header variable.  Controls the default height for a swept solid object created with the POLYSOLID command.  Minimum AutoCAD version: R2007.
    pub last_poly_solid_height: f64,
    /// The $LOFTANG1 header variable.  Sets the draft angle through the first cross section in a loft operation.  Minimum AutoCAD version: R2007.
    pub loft_operation_first_draft_angle: f64,
    /// The $LOFTANG2 header variable.  Sets the draft angle through the second cross section in a loft operation.  Minimum AutoCAD version: R2007.
    pub loft_operation_second_draft_angle: f64,
    /// The $LOFTMAG1 header variable.  Sets the magnitude of the draft angle through the first cross section in a loft operation.  Minimum AutoCAD version: R2007.
    pub loft_operation_first_magnitude: f64,
    /// The $LOFTMAG2 header variable.  Sets the magnitude of the draft angle through the second cross section in a loft operation.  Minimum AutoCAD version: R2007.
    pub loft_operation_second_magnitude: f64,
    /// The $LOFTPARAM header variable.  Controls the shape of lofted solids and surfaces.  Minimum AutoCAD version: R2007.
    pub loft_flags: i32,
    /// The $LOFTNORMALS header variable.  Controls the normals of a lofted object where it passes through cross sections.  Minimum AutoCAD version: R2007.
    pub lofted_object_normal_mode: LoftedObjectNormalMode,
    /// The $LATITUDE header variable.  The latitude of the geographic location assigned to the drawing.  Minimum AutoCAD version: R2007.
    pub latitude: f64,
    /// The $LONGITUDE header variable.  The longitude of the geographic location assigned to the drawing.  Minimum AutoCAD version: R2007.
    pub longitude: f64,
    /// The $NORTHDIRECTION header variable.  Specifies the angle between the Y axis of WCS and the grid north.  Minimum AutoCAD version: R2007.
    pub angle_between_y_axis_and_north: f64,
    /// The $TIMEZONE header variable.  Sets the time zone for the sun in the drawing.  Minimum AutoCAD version: R2007.
    pub time_zone: DrawingTimeZone,
    /// The $LIGHTGLYPHDISPLAY header variable.  Turns on and off the display of light glyphs.  Minimum AutoCAD version: R2007.
    pub use_light_glyph_display: bool,
    /// The $TILEMODELIGHTSYNCH header variable.  Unknown.  Minimum AutoCAD version: R2007.
    pub use_tile_mode_light_sync: bool,
    /// The $CMATERIAL header variable.  Sets the material of new objects.  Minimum AutoCAD version: R2007.
    pub current_material_handle: Handle,
    /// The $SOLIDHIST header variable.  Controls whether new composite solids retain a history of their original components.  Minimum AutoCAD version: R2007.
    pub new_solids_contain_history: bool,
    /// The $SHOWHIST header variable.  Controls the Show History property for solids in a drawing.  Minimum AutoCAD version: R2007.
    pub solid_history_mode: SolidHistoryMode,
    /// The $DWFFRAME header variable.  Determines whether DWF or DWFx underlay frames are visible or plotted in the current drawing.  Minimum AutoCAD version: R2007.
    pub dwf_underlay_frame_mode: UnderlayFrameMode,
    /// The $DGNFRAME header variable.  Determines whether DGN underlay frames are visible or plotted in the current drawing.  Minimum AutoCAD version: R2007.
    pub dgn_underlay_frame_mode: UnderlayFrameMode,
    /// The $REALWORLDSCALE header variable.  Drawing is scaled to the real world.  Minimum AutoCAD version: R2007.
    pub use_real_world_scale: bool,
    /// The $INTERFERECOLOR header variable.  Represents the ACI color index of the "interference objects" created during the interfere command.  Minimum AutoCAD version: R2007.
    pub interference_object_color: Color,
    /// The $INTERFEREOBJVS header variable.  Hard-pointer ID to the visual stype for interference objects.  Minimum AutoCAD version: R2007.
    pub interference_object_visual_style_pointer: Handle,
    /// The $INTERFEREVPVS header variable.  Hard-pointer ID to the visual styoe for the viewport during interference checking.  Minimum AutoCAD version: R2007.
    pub interference_view_port_visual_style_pointer: Handle,
    /// The $CSHADOW header variable.  Shadow mode for a 3D object.  Minimum AutoCAD version: R2007.
    pub shadow_mode: ShadowMode,
    /// The $SHADOWPLANELOCATION header variable.  Location of the ground shadow plane.  This is a Z axis ordinate.  Minimum AutoCAD version: R2007.
    pub shadow_plane_z_offset: f64,
    /// The $AXISMODE header variable.  Axis on.  Maximum AutoCAD version: R10.
    pub axis_on: bool,
    /// The $AXISUNIT header variable.  Axis X and Y tick spacing.  Maximum AutoCAD version: R10.
    pub axis_tick_spacing: Vector,
    /// The $FASTZOOM header variable.  Fast zoom enabled.  Maximum AutoCAD version: R10.
    pub fast_zoom: bool,
    /// The $GRIDMODE header variable.  Grid mode on.  Maximum AutoCAD version: R10.
    pub grid_on: bool,
    /// The $GRIDUNIT header variable.  Grid X and Y spacing.  Maximum AutoCAD version: R10.
    pub grid_spacing: Vector,
    /// The $SNAPANG header variable.  Snap grid rotation angle.  Maximum AutoCAD version: R10.
    pub snap_rotation_angle: f64,
    /// The $SNAPBASE header variable.  Snap/grid/base point (in UCS).  Maximum AutoCAD version: R10.
    pub snap_base_point: Point,
    /// The $SNAPISOPAIR header variable.  Isometric plane.  Maximum AutoCAD version: R10.
    pub snap_isometric_plane: SnapIsometricPlane,
    /// The $SNAPMODE header variable.  Snap mode on.  Maximum AutoCAD version: R10.
    pub snap_on: bool,
    /// The $SNAPSTYLE header variable.  Snap style.  Maximum AutoCAD version: R10.
    pub snap_style: SnapStyle,
    /// The $SNAPUNIT header variable.  Snap grid X and Y spacing.  Maximum AutoCAD version: R10.
    pub snap_spacing: Vector,
    /// The $VIEWCTR header variable.  XY center of current view on screen.  Maximum AutoCAD version: R10.
    pub view_center: Point,
    /// The $VIEWDIR header variable.  Viewing direction (direction from target in WCS).  Maximum AutoCAD version: R10.
    pub view_direction: Vector,
    /// The $VIEWSIZE header variable.  Height of view.  Maximum AutoCAD version: R10.
    pub view_height: f64,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: AcadVersion::R12, // $ACADVER
            maintenance_version: 0, // $ACADMAINTVER
            drawing_code_page: String::from("ANSI_1252"), // $DWGCODEPAGE
            last_saved_by: String::new(), // $LASTSAVEDBY
            required_versions: 0, // $REQUIREDVERSIONS
            insertion_base: Point::origin(), // $INSBASE
            minimum_drawing_extents: Point::origin(), // $EXTMIN
            maximum_drawing_extents: Point::origin(), // $EXTMAX
            minimum_drawing_limits: Point::origin(), // $LIMMIN
            maximum_drawing_limits: Point::new(12.0, 9.0, 0.0), // $LIMMAX
            draw_orthogonal_lines: false, // $ORTHOMODE
            use_regen_mode: true, // $REGENMODE
            fill_mode_on: true, // $FILLMODE
            use_quick_text_mode: false, // $QTEXTMODE
            mirror_text: false, // $MIRRTEXT
            drag_mode: DragMode::Auto, // $DRAGMODE
            line_type_scale: 1.0, // $LTSCALE
            object_snap_flags: 37, // $OSMODE
            attribute_visibility: AttributeVisibility::Normal, // $ATTMODE
            default_text_height: 0.2, // $TEXTSIZE
            trace_width: 0.05, // $TRACEWID
            text_style: String::from("STANDARD"), // $TEXTSTYLE
            current_layer: String::from("0"), // $CLAYER
            current_entity_line_type: String::from("BYLAYER"), // $CELTYPE
            current_entity_color: Color::by_layer(), // $CECOLOR
            current_entity_line_type_scale: 1.0, // $CELTSCALE
            retain_deleted_objects: true, // $DELOBJ
            display_silhouette_curves_in_wireframe_mode: false, // $DISPSILH
            solid_visual_style_pointer: Handle::empty(), // $DRAGVS
            dimensioning_scale_factor: 1.0, // $DIMSCALE
            dimensioning_arrow_size: 0.18, // $DIMASZ
            dimension_extension_line_offset: 0.0625, // $DIMEXO
            dimension_line_increment: 0.38, // $DIMDLI
            dimension_distance_rounding_value: 0.0, // $DIMRND
            dimension_line_extension: 0.0, // $DIMDLE
            dimension_extension_line_extension: 0.18, // $DIMEXE
            dimension_plus_tolerance: 0.0, // $DIMTP
            dimension_minus_tolerance: 0.0, // $DIMTM
            dimensioning_text_height: 0.18, // $DIMTXT
            center_mark_size: 0.09, // $DIMCEN
            dimensioning_tick_size: 0.0, // $DIMTSZ
            generate_dimension_tolerances: false, // $DIMTOL
            generate_dimension_limits: false, // $DIMLIM
            dimension_text_inside_horizontal: true, // $DIMTIH
            dimension_text_outside_horizontal: true, // $DIMTOH
            suppress_first_dimension_extension_line: false, // $DIMSE1
            suppress_second_dimension_extension_line: false, // $DIMSE2
            text_above_dimension_line: false, // $DIMTAD
            dimension_unit_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches, // $DIMZIN
            arrow_block_name: String::new(), // $DIMBLK
            create_associative_dimensioning: true, // $DIMASO
            recompute_dimensions_while_dragging: true, // $DIMSHO
            dimensioning_suffix: String::new(), // $DIMPOST
            alternate_dimensioning_suffix: String::new(), // $DIMAPOST
            use_alternate_dimensioning: false, // $DIMALT
            alternate_dimensioning_decimal_places: 2, // $DIMALTD
            alternate_dimensioning_scale_factor: 25.4, // $DIMALTF
            dimension_linear_measurements_scale_factor: 1.0, // $DIMLFAC
            force_dimension_line_extensions_outside_if_text_is: false, // $DIMTOFL
            dimension_vertical_text_position: 0.0, // $DIMTVP
            force_dimension_text_inside_extensions: false, // $DIMTIX
            suppress_outside_extension_dimension_lines: false, // $DIMSOXD
            use_separate_arrow_blocks_for_dimensions: false, // $DIMSAH
            first_arrow_block_name: String::new(), // $DIMBLK1
            second_arrow_block_name: String::new(), // $DIMBLK2
            dimension_style_name: String::from("STANDARD"), // $DIMSTYLE
            dimension_line_color: Color::by_block(), // $DIMCLRD
            dimension_extension_line_color: Color::by_block(), // $DIMCLRE
            dimension_text_color: Color::by_block(), // $DIMCLRT
            dimension_tolerance_display_scale_factor: 1.0, // $DIMTFAC
            dimension_line_gap: 0.09, // $DIMGAP
            dimension_text_justification: DimensionTextJustification::AboveLineCenter, // $DIMJUST
            dimension_tolerance_vertical_justification: Justification::Middle, // $DIMTOLJ
            dimension_tolerance_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches, // $DIMTZIN
            alternate_dimensioning_zero_supression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches, // $DIMALTZ
            alternate_dimensioning_tolerance_zero_supression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches, // $DIMALTTZ
            dimension_text_and_arrow_placement: DimensionFit::TextAndArrowsOutsideLines, // $DIMFIT
            dimension_cursor_controls_text_position: false, // $DIMUPT
            dimension_unit_format: UnitFormat::Decimal, // $DIMUNIT
            dimension_unit_tolerance_decimal_places: 4, // $DIMDEC
            dimension_tolerance_decimal_places: 4, // $DIMTDEC
            alternate_dimensioning_units: UnitFormat::Decimal, // $DIMALTU
            alternate_dimensioning_tolerance_decimal_places: 2, // $DIMALTTD
            dimension_text_style: String::from("STANDARD"), // $DIMTXSTY
            dimensioning_angle_format: AngleFormat::DecimalDegrees, // $DIMAUNIT
            angular_dimension_precision: 0, // $DIMADEC
            alternate_dimensioning_unit_rounding: 0.0, // $DIMALTRND
            dimension_angle_zero_suppression: UnitZeroSuppression::SuppressZeroFeetAndZeroInches, // $DIMAZIN
            dimension_decimal_separator_char: '.', // $DIMDSEP
            dimension_text_height_scale_factor: DimensionFractionFormat::HorizontalStacking, // $DIMFRAC
            dimension_leader_block_name: String::new(), // $DIMLDRBLK
            dimension_non_angular_units: NonAngularUnits::Decimal, // $DIMLUNIT
            dimension_line_weight: LineWeight::by_layer(), // $DIMLWD
            dimension_extension_line_weight: LineWeight::by_layer(), // $DIMLWE
            dimension_text_movement_rule: DimensionTextMovementRule::MoveLineWithText, // $DIMTMOVE
            dimension_line_fixed_length: 1.0, // $DIMFXL
            dimension_line_fixed_length_on: false, // $DIMFXLON
            dimension_transverse_segment_angle_in_jogged_radius: ::std::f64::consts::PI / 4.0, // $DIMJOGANG
            dimension_text_background_color_mode: DimensionTextBackgroundColorMode::None, // $DIMTFILL
            dxf_dimension_text_background_custom_color: Color::by_block(), // $DIMTFILLCLR
            dimension_arc_symbol_display_mode: DimensionArcSymbolDisplayMode::SymbolBeforeText, // $DIMARCSYM
            dimension_line_type: String::from("BYLAYER"), // $DIMLTYPE
            dimension_first_extension_line_type: String::new(), // $DIMLTEX1
            dimension_second_extension_line_type: String::new(), // $DIMLTEX2
            dimension_text_direction: TextDirection::LeftToRight, // $DIMTXTDIRECTION
            unit_format: UnitFormat::Decimal, // $LUNITS
            unit_precision: 4, // $LUPREC
            sketch_record_increment: 0.1, // $SKETCHINC
            fillet_radius: 0.0, // $FILLETRAD
            angle_unit_format: AngleFormat::DecimalDegrees, // $AUNITS
            angle_unit_precision: 0, // $AUPREC
            file_name: String::from("."), // $MENU
            elevation: 0.0, // $ELEVATION
            paperspace_elevation: 0.0, // $PELEVATION
            thickness: 0.0, // $THICKNESS
            use_limits_checking: false, // $LIMCHECK
            blip_mode: false, // $BLIPMODE
            first_chamfer_distance: 0.0, // $CHAMFERA
            second_chamfer_distance: 0.0, // $CHAMFERB
            chamfer_length: 0.0, // $CHAMFERC
            chamfer_angle: 0.0, // $CHAMFERD
            polyline_sketch_mode: PolySketchMode::SketchLines, // $SKPOLY
            creation_date: Local::now(), // $TDCREATE
            creation_date_universal: Utc::now(), // $TDUCREATE
            update_date: Local::now(), // $TDUPDATE
            update_date_universal: Utc::now(), // $TDUUPDATE
            time_in_drawing: Duration::default(), // $TDINDWG
            user_elapsed_timer: Duration::default(), // $TDUSRTIMER
            user_timer_on: true, // $USRTIMER
            angle_zero_direction: 0.0, // $ANGBASE
            angle_direction: AngleDirection::CounterClockwise, // $ANGDIR
            point_display_mode: 0, // $PDMODE
            point_display_size: 0.0, // $PDSIZE
            default_polyline_width: 0.0, // $PLINEWID
            coordinate_display: CoordinateDisplay::ContinuousUpdate, // $COORDS
            display_spline_polygon_control: false, // $SPLFRAME
            pedit_spline_curve_type: PolylineCurvedAndSmoothSurfaceType::CubicBSpline, // $SPLINETYPE
            line_segments_per_spline_patch: 8, // $SPLINESEGS
            show_attribute_entry_dialogs: true, // $ATTDIA
            prompt_for_attribute_on_insert: true, // $ATTREQ
            handles_enabled: true, // $HANDLING
            next_available_handle: Handle(1), // $HANDSEED
            mesh_tabulations_in_first_direction: 6, // $SURFTAB1
            mesh_tabulations_in_second_direction: 6, // $SURFTAB2
            pedit_smooth_surface_type: PolylineCurvedAndSmoothSurfaceType::CubicBSpline, // $SURFTYPE
            pedit_smooth_m_densith: 6, // $SURFU
            pedit_smooth_n_densith: 6, // $SURFV
            ucs_definition_name: String::new(), // $UCSBASE
            ucs_name: String::new(), // $UCSNAME
            ucs_origin: Point::origin(), // $UCSORG
            ucs_x_axis: Vector::x_axis(), // $UCSXDIR
            ucs_y_axis: Vector::y_axis(), // $UCSYDIR
            ortho_ucs_reference: String::new(), // $UCSORTHOREF
            orthgraphic_view_type: OrthographicViewType::None, // $UCSORTHOVIEW
            ucs_origin_top: Point::origin(), // $UCSORGTOP
            ucs_origin_bottom: Point::origin(), // $UCSORGBOTTOM
            ucs_origin_left: Point::origin(), // $UCSORGLEFT
            ucs_origin_right: Point::origin(), // $UCSORGRIGHT
            ucs_origin_front: Point::origin(), // $UCSORGFRONT
            ucs_origin_back: Point::origin(), // $UCSORGBACK
            paperspace_ucs_definition_name: String::new(), // $PUCSBASE
            paperspace_ucs_name: String::new(), // $PUCSNAME
            paperspace_ucs_origin: Point::origin(), // $PUCSORG
            paperspace_x_axis: Vector::x_axis(), // $PUCSXDIR
            paperspace_y_axis: Vector::y_axis(), // $PUCSYDIR
            paperspace_ortho_ucs_reference: String::new(), // $PUCSORTHOREF
            paperspace_orthographic_view_type: OrthographicViewType::None, // $PUCSORTHOVIEW
            paperspace_ucs_origin_top: Point::origin(), // $PUCSORGTOP
            paperspace_ucs_origin_bottom: Point::origin(), // $PUCSORGBOTTOM
            paperspace_ucs_origin_left: Point::origin(), // $PUCSORGLEFT
            paperspace_ucs_origin_right: Point::origin(), // $PUCSORGRIGHT
            paperspace_ucs_origin_front: Point::origin(), // $PUCSORGFRONT
            paperspace_ucs_origin_back: Point::origin(), // $PUCSORGBACK
            user_int1: 0, // $USERI1
            user_int2: 0, // $USERI2
            user_int3: 0, // $USERI3
            user_int4: 0, // $USERI4
            user_int5: 0, // $USERI5
            user_real1: 0.0, // $USERR1
            user_real2: 0.0, // $USERR2
            user_real3: 0.0, // $USERR3
            user_real4: 0.0, // $USERR4
            user_real5: 0.0, // $USERR5
            set_ucs_to_wcs_in_d_view_or_v_point: true, // $WORLDVIEW
            edge_shading: ShadeEdgeMode::FacesInEntityColorEdgesInBlack, // $SHADEDGE
            percent_ambient_to_diffuse: 70, // $SHADEDIF
            previous_release_tile_compatability: true, // $TILEMODE
            maximum_active_viewports: 64, // $MAXACTVP
            paperspace_insertion_base: Point::origin(), // $PINSBASE
            limit_checking_in_paperspace: false, // $PLIMCHECK
            paperspace_minimum_drawing_extents: Point::new(1.0e20, 1.0e20, 1.0e20), // $PEXTMIN
            paperspace_maximum_drawing_extents: Point::new(-1.0e20, -1.0e20, -1.0e20), // $PEXTMAX
            paperspace_minimum_drawing_limits: Point::origin(), // $PLIMMIN
            paperspace_maximum_drawing_limits: Point::new(12.0, 9.0, 0.0), // $PLIMMAX
            display_fractions_in_input: false, // $UNITMODE
            retain_x_ref_dependent_visibility_settings: true, // $VISRETAIN
            is_polyline_continuous_around_verticies: false, // $PLINEGEN
            scale_line_types_in_paperspace: true, // $PSLTSCALE
            spacial_index_max_depth: 3020, // $TREEDEPTH
            pick_style: PickStyle::Group, // $PICKSTYLE
            current_multiline_style: String::from("STANDARD"), // $CMLSTYLE
            current_multiline_justification: Justification::Top, // $CMLJUST
            current_multiline_scale: 1.0, // $CMLSCALE
            save_proxy_graphics: true, // $PROXYGRAPHICS
            drawing_units: DrawingUnits::English, // $MEASUREMENT
            new_object_line_weight: LineWeight::by_block(), // $CELWEIGHT
            end_cap_setting: EndCapSetting::None, // $ENDCAPS
            lineweight_joint_setting: JoinStyle::None, // $JOINSTYLE
            display_linewieght_in_model_and_layout_tab: false, // $LWDISPLAY
            default_drawing_units: Units::Unitless, // $INSUNITS
            hyperlink_base: String::new(), // $HYPERLINKBASE
            stylesheet: String::new(), // $STYLESHEET
            can_use_in_place_reference_editing: true, // $XEDIT
            new_object_plot_style_handle: Handle::empty(), // $CEPSNID
            new_object_plot_style: PlotStyle::ByLayer, // $CEPSNTYPE
            uses_color_dependent_plot_style_tables: true, // $PSTYLEMODE
            fingerprint_guid: Uuid::new_v4(), // $FINGERPRINTGUID
            version_guid: Uuid::new_v4(), // $VERSIONGUID
            use_acad2000_symbol_table_naming: true, // $EXTNAMES
            viewport_view_scale_factor: 0.0, // $PSVPSCALE
            ole_startup: false, // $OLESTARTUP
            object_sorting_methods_flags: 127, // $SORTENTS
            layer_and_spatial_index_save_mode: LayerAndSpatialIndexSaveMode::None, // $INDEXCTL
            hide_text_objects_when_producint_hidden_view: false, // $HIDETEXT
            is_x_ref_clipping_boundary_visible: XrefClippingBoundaryVisibility::DisplayedNotPlotted, // $XCLIPFRAME
            halo_gap_percent: 0.0, // $HALOGAP
            obscured_line_color: Color::by_entity(), // $OBSCOLOR
            obscured_line_type_style: LineTypeStyle::Off, // $OBSLTYPE
            display_intersection_polylines: false, // $INTERSECTIONDISPLAY
            intersection_polyline_color: Color::by_entity(), // $INTERSECTIONCOLOR
            dimension_object_associativity: DimensionAssociativity::NonAssociativeObjects, // $DIMASSOC
            project_name: String::new(), // $PROJECTNAME
            use_camera_display: false, // $CAMERADISPLAY
            lens_length: 50.0, // $LENSLENGTH
            camera_height: 0.0, // $CAMERAHEIGHT
            steps_per_second_in_walk_or_fly_mode: 2.0, // $STEPSPERSEC
            step_size_in_walk_or_fly_mode: 6.0, // $STEPSIZE
            dwf_3d_precision: Dwf3DPrecision::Deviation_0_5, // $3DDWFPREC
            last_poly_solid_width: 0.25, // $PSOLWIDTH
            last_poly_solid_height: 4.0, // $PSOLHEIGHT
            loft_operation_first_draft_angle: ::std::f64::consts::PI / 2.0, // $LOFTANG1
            loft_operation_second_draft_angle: ::std::f64::consts::PI / 2.0, // $LOFTANG2
            loft_operation_first_magnitude: 0.0, // $LOFTMAG1
            loft_operation_second_magnitude: 0.0, // $LOFTMAG2
            loft_flags: 7, // $LOFTPARAM
            lofted_object_normal_mode: LoftedObjectNormalMode::SmoothFit, // $LOFTNORMALS
            latitude: 37.7950, // $LATITUDE
            longitude: -122.3940, // $LONGITUDE
            angle_between_y_axis_and_north: 0.0, // $NORTHDIRECTION
            time_zone: DrawingTimeZone::PacificTime_US_Canada_SanFrancisco_Vancouver, // $TIMEZONE
            use_light_glyph_display: true, // $LIGHTGLYPHDISPLAY
            use_tile_mode_light_sync: true, // $TILEMODELIGHTSYNCH
            current_material_handle: Handle::empty(), // $CMATERIAL
            new_solids_contain_history: false, // $SOLIDHIST
            solid_history_mode: SolidHistoryMode::DoesNotOverride, // $SHOWHIST
            dwf_underlay_frame_mode: UnderlayFrameMode::DisplayNoPlot, // $DWFFRAME
            dgn_underlay_frame_mode: UnderlayFrameMode::None, // $DGNFRAME
            use_real_world_scale: true, // $REALWORLDSCALE
            interference_object_color: Color::from_index(1), // $INTERFERECOLOR
            interference_object_visual_style_pointer: Handle::empty(), // $INTERFEREOBJVS
            interference_view_port_visual_style_pointer: Handle::empty(), // $INTERFEREVPVS
            shadow_mode: ShadowMode::CastsAndReceivesShadows, // $CSHADOW
            shadow_plane_z_offset: 0.0, // $SHADOWPLANELOCATION
            axis_on: false, // $AXISMODE
            axis_tick_spacing: Vector::zero(), // $AXISUNIT
            fast_zoom: true, // $FASTZOOM
            grid_on: false, // $GRIDMODE
            grid_spacing: Vector::new(1.0, 1.0, 0.0), // $GRIDUNIT
            snap_rotation_angle: 0.0, // $SNAPANG
            snap_base_point: Point::origin(), // $SNAPBASE
            snap_isometric_plane: SnapIsometricPlane::Left, // $SNAPISOPAIR
            snap_on: false, // $SNAPMODE
            snap_style: SnapStyle::Standard, // $SNAPSTYLE
            snap_spacing: Vector::new(1.0, 1.0, 0.0), // $SNAPUNIT
            view_center: Point::origin(), // $VIEWCTR
            view_direction: Vector::z_axis(), // $VIEWDIR
            view_height: 1.0, // $VIEWSIZE
        }
    }
}

impl Header {
    // object_snap_flags flags
    /// Snap to line segment endpoints.  Maximum AutoCAD version: R14.
    pub fn end_point_snap(&self) -> bool {
        self.object_snap_flags & 1 != 0
    }
    /// Snap to line segment endpoints.  Maximum AutoCAD version: R14.
    pub fn set_end_point_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 1;
        }
        else {
            self.object_snap_flags &= !1;
        }
    }
    /// Snap to line segment midpoints.  Maximum AutoCAD version: R14.
    pub fn mid_point_snap(&self) -> bool {
        self.object_snap_flags & 2 != 0
    }
    /// Snap to line segment midpoints.  Maximum AutoCAD version: R14.
    pub fn set_mid_point_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 2;
        }
        else {
            self.object_snap_flags &= !2;
        }
    }
    /// Snap to circle and arc center points.  Maximum AutoCAD version: R14.
    pub fn center_snap(&self) -> bool {
        self.object_snap_flags & 4 != 0
    }
    /// Snap to circle and arc center points.  Maximum AutoCAD version: R14.
    pub fn set_center_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 4;
        }
        else {
            self.object_snap_flags &= !4;
        }
    }
    /// Snap to nodes.  Maximum AutoCAD version: R14.
    pub fn node_snap(&self) -> bool {
        self.object_snap_flags & 8 != 0
    }
    /// Snap to nodes.  Maximum AutoCAD version: R14.
    pub fn set_node_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 8;
        }
        else {
            self.object_snap_flags &= !8;
        }
    }
    /// Snap to circle quadrants.  Maximum AutoCAD version: R14.
    pub fn quadrant_snap(&self) -> bool {
        self.object_snap_flags & 16 != 0
    }
    /// Snap to circle quadrants.  Maximum AutoCAD version: R14.
    pub fn set_quadrant_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 16;
        }
        else {
            self.object_snap_flags &= !16;
        }
    }
    /// Snap to segment intersections.  Maximum AutoCAD version: R14.
    pub fn intersection_snap(&self) -> bool {
        self.object_snap_flags & 32 != 0
    }
    /// Snap to segment intersections.  Maximum AutoCAD version: R14.
    pub fn set_intersection_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 32;
        }
        else {
            self.object_snap_flags &= !32;
        }
    }
    /// Snap to block insertion points.  Maximum AutoCAD version: R14.
    pub fn insertion_snap(&self) -> bool {
        self.object_snap_flags & 64 != 0
    }
    /// Snap to block insertion points.  Maximum AutoCAD version: R14.
    pub fn set_insertion_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 64;
        }
        else {
            self.object_snap_flags &= !64;
        }
    }
    /// Snap to perpendicular points.  Maximum AutoCAD version: R14.
    pub fn perpendicular_snap(&self) -> bool {
        self.object_snap_flags & 128 != 0
    }
    /// Snap to perpendicular points.  Maximum AutoCAD version: R14.
    pub fn set_perpendicular_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 128;
        }
        else {
            self.object_snap_flags &= !128;
        }
    }
    /// Snap to tangent points.  Maximum AutoCAD version: R14.
    pub fn tangent_snap(&self) -> bool {
        self.object_snap_flags & 256 != 0
    }
    /// Snap to tangent points.  Maximum AutoCAD version: R14.
    pub fn set_tangent_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 256;
        }
        else {
            self.object_snap_flags &= !256;
        }
    }
    /// Snap to nearest object points.  Maximum AutoCAD version: R14.
    pub fn nearest_snap(&self) -> bool {
        self.object_snap_flags & 512 != 0
    }
    /// Snap to nearest object points.  Maximum AutoCAD version: R14.
    pub fn set_nearest_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 512;
        }
        else {
            self.object_snap_flags &= !512;
        }
    }
    /// Snap to apparent intersection points.  Maximum AutoCAD version: R14.
    pub fn apparent_intersection_snap(&self) -> bool {
        self.object_snap_flags & 2048 != 0
    }
    /// Snap to apparent intersection points.  Maximum AutoCAD version: R14.
    pub fn set_apparent_intersection_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 2048;
        }
        else {
            self.object_snap_flags &= !2048;
        }
    }
    /// Snap to extension points.  Maximum AutoCAD version: R14.
    pub fn extension_snap(&self) -> bool {
        self.object_snap_flags & 4096 != 0
    }
    /// Snap to extension points.  Maximum AutoCAD version: R14.
    pub fn set_extension_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 4096;
        }
        else {
            self.object_snap_flags &= !4096;
        }
    }
    /// Snap to parallel points.  Maximum AutoCAD version: R14.
    pub fn parallel_snap(&self) -> bool {
        self.object_snap_flags & 8192 != 0
    }
    /// Snap to parallel points.  Maximum AutoCAD version: R14.
    pub fn set_parallel_snap(&mut self, val: bool) {
        if val {
            self.object_snap_flags |= 8192;
        }
        else {
            self.object_snap_flags &= !8192;
        }
    }
    // object_sorting_methods_flags flags
    /// Sorts for object selection.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_object_selection(&self) -> bool {
        self.object_sorting_methods_flags & 1 != 0
    }
    /// Sorts for object selection.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_object_selection(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 1;
        }
        else {
            self.object_sorting_methods_flags &= !1;
        }
    }
    /// Sorts for object snap.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_object_snap(&self) -> bool {
        self.object_sorting_methods_flags & 2 != 0
    }
    /// Sorts for object snap.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_object_snap(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 2;
        }
        else {
            self.object_sorting_methods_flags &= !2;
        }
    }
    /// Sorts for redraws.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_redraw(&self) -> bool {
        self.object_sorting_methods_flags & 4 != 0
    }
    /// Sorts for redraws.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_redraw(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 4;
        }
        else {
            self.object_sorting_methods_flags &= !4;
        }
    }
    /// Sorts for MSLIDE command slide creation.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_mslide(&self) -> bool {
        self.object_sorting_methods_flags & 8 != 0
    }
    /// Sorts for MSLIDE command slide creation.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_mslide(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 8;
        }
        else {
            self.object_sorting_methods_flags &= !8;
        }
    }
    /// Sorts for REGEN commands.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_regen(&self) -> bool {
        self.object_sorting_methods_flags & 16 != 0
    }
    /// Sorts for REGEN commands.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_regen(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 16;
        }
        else {
            self.object_sorting_methods_flags &= !16;
        }
    }
    /// Sorts for plotting.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_plotting(&self) -> bool {
        self.object_sorting_methods_flags & 32 != 0
    }
    /// Sorts for plotting.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_plotting(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 32;
        }
        else {
            self.object_sorting_methods_flags &= !32;
        }
    }
    /// Sorts for PostScript output.  Minimum AutoCAD version: R2004.
    pub fn sort_objects_for_post_script_output(&self) -> bool {
        self.object_sorting_methods_flags & 64 != 0
    }
    /// Sorts for PostScript output.  Minimum AutoCAD version: R2004.
    pub fn set_sort_objects_for_post_script_output(&mut self, val: bool) {
        if val {
            self.object_sorting_methods_flags |= 64;
        }
        else {
            self.object_sorting_methods_flags &= !64;
        }
    }
    // loft_flags flags
    /// No twist (minimizes the twist between cross sections).  Minimum AutoCAD version: R2007.
    pub fn no_twist(&self) -> bool {
        self.loft_flags & 1 != 0
    }
    /// No twist (minimizes the twist between cross sections).  Minimum AutoCAD version: R2007.
    pub fn set_no_twist(&mut self, val: bool) {
        if val {
            self.loft_flags |= 1;
        }
        else {
            self.loft_flags &= !1;
        }
    }
    /// Align direction (aligns the start to end direction of each cross section curve).  Minimum AutoCAD version: R2007.
    pub fn align_direction(&self) -> bool {
        self.loft_flags & 2 != 0
    }
    /// Align direction (aligns the start to end direction of each cross section curve).  Minimum AutoCAD version: R2007.
    pub fn set_align_direction(&mut self, val: bool) {
        if val {
            self.loft_flags |= 2;
        }
        else {
            self.loft_flags &= !2;
        }
    }
    /// Simplify (produces simple solids and surfaces, such as a cylinder or plane, instead of spline solids and surfaces).  Minimum AutoCAD version: R2007.
    pub fn simplify(&self) -> bool {
        self.loft_flags & 4 != 0
    }
    /// Simplify (produces simple solids and surfaces, such as a cylinder or plane, instead of spline solids and surfaces).  Minimum AutoCAD version: R2007.
    pub fn set_simplify(&mut self, val: bool) {
        if val {
            self.loft_flags |= 4;
        }
        else {
            self.loft_flags &= !4;
        }
    }
    /// Close (closes the surface or solid between the first and the last cross sections).  Minimum AutoCAD version: R2007.
    pub fn close(&self) -> bool {
        self.loft_flags & 8 != 0
    }
    /// Close (closes the surface or solid between the first and the last cross sections).  Minimum AutoCAD version: R2007.
    pub fn set_close(&mut self, val: bool) {
        if val {
            self.loft_flags |= 8;
        }
        else {
            self.loft_flags &= !8;
        }
    }
    /// Sets the default values on the header.
    pub fn set_defaults(&mut self) {
        self.version = AcadVersion::R12; // $ACADVER
        self.maintenance_version = 0; // $ACADMAINTVER
        self.drawing_code_page = String::from("ANSI_1252"); // $DWGCODEPAGE
        self.last_saved_by = String::new(); // $LASTSAVEDBY
        self.required_versions = 0; // $REQUIREDVERSIONS
        self.insertion_base = Point::origin(); // $INSBASE
        self.minimum_drawing_extents = Point::origin(); // $EXTMIN
        self.maximum_drawing_extents = Point::origin(); // $EXTMAX
        self.minimum_drawing_limits = Point::origin(); // $LIMMIN
        self.maximum_drawing_limits = Point::new(12.0, 9.0, 0.0); // $LIMMAX
        self.draw_orthogonal_lines = false; // $ORTHOMODE
        self.use_regen_mode = true; // $REGENMODE
        self.fill_mode_on = true; // $FILLMODE
        self.use_quick_text_mode = false; // $QTEXTMODE
        self.mirror_text = false; // $MIRRTEXT
        self.drag_mode = DragMode::Auto; // $DRAGMODE
        self.line_type_scale = 1.0; // $LTSCALE
        self.object_snap_flags = 37; // $OSMODE
        self.attribute_visibility = AttributeVisibility::Normal; // $ATTMODE
        self.default_text_height = 0.2; // $TEXTSIZE
        self.trace_width = 0.05; // $TRACEWID
        self.text_style = String::from("STANDARD"); // $TEXTSTYLE
        self.current_layer = String::from("0"); // $CLAYER
        self.current_entity_line_type = String::from("BYLAYER"); // $CELTYPE
        self.current_entity_color = Color::by_layer(); // $CECOLOR
        self.current_entity_line_type_scale = 1.0; // $CELTSCALE
        self.retain_deleted_objects = true; // $DELOBJ
        self.display_silhouette_curves_in_wireframe_mode = false; // $DISPSILH
        self.solid_visual_style_pointer = Handle::empty(); // $DRAGVS
        self.dimensioning_scale_factor = 1.0; // $DIMSCALE
        self.dimensioning_arrow_size = 0.18; // $DIMASZ
        self.dimension_extension_line_offset = 0.0625; // $DIMEXO
        self.dimension_line_increment = 0.38; // $DIMDLI
        self.dimension_distance_rounding_value = 0.0; // $DIMRND
        self.dimension_line_extension = 0.0; // $DIMDLE
        self.dimension_extension_line_extension = 0.18; // $DIMEXE
        self.dimension_plus_tolerance = 0.0; // $DIMTP
        self.dimension_minus_tolerance = 0.0; // $DIMTM
        self.dimensioning_text_height = 0.18; // $DIMTXT
        self.center_mark_size = 0.09; // $DIMCEN
        self.dimensioning_tick_size = 0.0; // $DIMTSZ
        self.generate_dimension_tolerances = false; // $DIMTOL
        self.generate_dimension_limits = false; // $DIMLIM
        self.dimension_text_inside_horizontal = true; // $DIMTIH
        self.dimension_text_outside_horizontal = true; // $DIMTOH
        self.suppress_first_dimension_extension_line = false; // $DIMSE1
        self.suppress_second_dimension_extension_line = false; // $DIMSE2
        self.text_above_dimension_line = false; // $DIMTAD
        self.dimension_unit_zero_suppression = UnitZeroSuppression::SuppressZeroFeetAndZeroInches; // $DIMZIN
        self.arrow_block_name = String::new(); // $DIMBLK
        self.create_associative_dimensioning = true; // $DIMASO
        self.recompute_dimensions_while_dragging = true; // $DIMSHO
        self.dimensioning_suffix = String::new(); // $DIMPOST
        self.alternate_dimensioning_suffix = String::new(); // $DIMAPOST
        self.use_alternate_dimensioning = false; // $DIMALT
        self.alternate_dimensioning_decimal_places = 2; // $DIMALTD
        self.alternate_dimensioning_scale_factor = 25.4; // $DIMALTF
        self.dimension_linear_measurements_scale_factor = 1.0; // $DIMLFAC
        self.force_dimension_line_extensions_outside_if_text_is = false; // $DIMTOFL
        self.dimension_vertical_text_position = 0.0; // $DIMTVP
        self.force_dimension_text_inside_extensions = false; // $DIMTIX
        self.suppress_outside_extension_dimension_lines = false; // $DIMSOXD
        self.use_separate_arrow_blocks_for_dimensions = false; // $DIMSAH
        self.first_arrow_block_name = String::new(); // $DIMBLK1
        self.second_arrow_block_name = String::new(); // $DIMBLK2
        self.dimension_style_name = String::from("STANDARD"); // $DIMSTYLE
        self.dimension_line_color = Color::by_block(); // $DIMCLRD
        self.dimension_extension_line_color = Color::by_block(); // $DIMCLRE
        self.dimension_text_color = Color::by_block(); // $DIMCLRT
        self.dimension_tolerance_display_scale_factor = 1.0; // $DIMTFAC
        self.dimension_line_gap = 0.09; // $DIMGAP
        self.dimension_text_justification = DimensionTextJustification::AboveLineCenter; // $DIMJUST
        self.dimension_tolerance_vertical_justification = Justification::Middle; // $DIMTOLJ
        self.dimension_tolerance_zero_suppression = UnitZeroSuppression::SuppressZeroFeetAndZeroInches; // $DIMTZIN
        self.alternate_dimensioning_zero_supression = UnitZeroSuppression::SuppressZeroFeetAndZeroInches; // $DIMALTZ
        self.alternate_dimensioning_tolerance_zero_supression = UnitZeroSuppression::SuppressZeroFeetAndZeroInches; // $DIMALTTZ
        self.dimension_text_and_arrow_placement = DimensionFit::TextAndArrowsOutsideLines; // $DIMFIT
        self.dimension_cursor_controls_text_position = false; // $DIMUPT
        self.dimension_unit_format = UnitFormat::Decimal; // $DIMUNIT
        self.dimension_unit_tolerance_decimal_places = 4; // $DIMDEC
        self.dimension_tolerance_decimal_places = 4; // $DIMTDEC
        self.alternate_dimensioning_units = UnitFormat::Decimal; // $DIMALTU
        self.alternate_dimensioning_tolerance_decimal_places = 2; // $DIMALTTD
        self.dimension_text_style = String::from("STANDARD"); // $DIMTXSTY
        self.dimensioning_angle_format = AngleFormat::DecimalDegrees; // $DIMAUNIT
        self.angular_dimension_precision = 0; // $DIMADEC
        self.alternate_dimensioning_unit_rounding = 0.0; // $DIMALTRND
        self.dimension_angle_zero_suppression = UnitZeroSuppression::SuppressZeroFeetAndZeroInches; // $DIMAZIN
        self.dimension_decimal_separator_char = '.'; // $DIMDSEP
        self.dimension_text_height_scale_factor = DimensionFractionFormat::HorizontalStacking; // $DIMFRAC
        self.dimension_leader_block_name = String::new(); // $DIMLDRBLK
        self.dimension_non_angular_units = NonAngularUnits::Decimal; // $DIMLUNIT
        self.dimension_line_weight = LineWeight::by_layer(); // $DIMLWD
        self.dimension_extension_line_weight = LineWeight::by_layer(); // $DIMLWE
        self.dimension_text_movement_rule = DimensionTextMovementRule::MoveLineWithText; // $DIMTMOVE
        self.dimension_line_fixed_length = 1.0; // $DIMFXL
        self.dimension_line_fixed_length_on = false; // $DIMFXLON
        self.dimension_transverse_segment_angle_in_jogged_radius = ::std::f64::consts::PI / 4.0; // $DIMJOGANG
        self.dimension_text_background_color_mode = DimensionTextBackgroundColorMode::None; // $DIMTFILL
        self.dxf_dimension_text_background_custom_color = Color::by_block(); // $DIMTFILLCLR
        self.dimension_arc_symbol_display_mode = DimensionArcSymbolDisplayMode::SymbolBeforeText; // $DIMARCSYM
        self.dimension_line_type = String::from("BYLAYER"); // $DIMLTYPE
        self.dimension_first_extension_line_type = String::new(); // $DIMLTEX1
        self.dimension_second_extension_line_type = String::new(); // $DIMLTEX2
        self.dimension_text_direction = TextDirection::LeftToRight; // $DIMTXTDIRECTION
        self.unit_format = UnitFormat::Decimal; // $LUNITS
        self.unit_precision = 4; // $LUPREC
        self.sketch_record_increment = 0.1; // $SKETCHINC
        self.fillet_radius = 0.0; // $FILLETRAD
        self.angle_unit_format = AngleFormat::DecimalDegrees; // $AUNITS
        self.angle_unit_precision = 0; // $AUPREC
        self.file_name = String::from("."); // $MENU
        self.elevation = 0.0; // $ELEVATION
        self.paperspace_elevation = 0.0; // $PELEVATION
        self.thickness = 0.0; // $THICKNESS
        self.use_limits_checking = false; // $LIMCHECK
        self.blip_mode = false; // $BLIPMODE
        self.first_chamfer_distance = 0.0; // $CHAMFERA
        self.second_chamfer_distance = 0.0; // $CHAMFERB
        self.chamfer_length = 0.0; // $CHAMFERC
        self.chamfer_angle = 0.0; // $CHAMFERD
        self.polyline_sketch_mode = PolySketchMode::SketchLines; // $SKPOLY
        self.creation_date = Local::now(); // $TDCREATE
        self.creation_date_universal = Utc::now(); // $TDUCREATE
        self.update_date = Local::now(); // $TDUPDATE
        self.update_date_universal = Utc::now(); // $TDUUPDATE
        self.time_in_drawing = Duration::default(); // $TDINDWG
        self.user_elapsed_timer = Duration::default(); // $TDUSRTIMER
        self.user_timer_on = true; // $USRTIMER
        self.angle_zero_direction = 0.0; // $ANGBASE
        self.angle_direction = AngleDirection::CounterClockwise; // $ANGDIR
        self.point_display_mode = 0; // $PDMODE
        self.point_display_size = 0.0; // $PDSIZE
        self.default_polyline_width = 0.0; // $PLINEWID
        self.coordinate_display = CoordinateDisplay::ContinuousUpdate; // $COORDS
        self.display_spline_polygon_control = false; // $SPLFRAME
        self.pedit_spline_curve_type = PolylineCurvedAndSmoothSurfaceType::CubicBSpline; // $SPLINETYPE
        self.line_segments_per_spline_patch = 8; // $SPLINESEGS
        self.show_attribute_entry_dialogs = true; // $ATTDIA
        self.prompt_for_attribute_on_insert = true; // $ATTREQ
        self.handles_enabled = true; // $HANDLING
        self.next_available_handle = Handle(1); // $HANDSEED
        self.mesh_tabulations_in_first_direction = 6; // $SURFTAB1
        self.mesh_tabulations_in_second_direction = 6; // $SURFTAB2
        self.pedit_smooth_surface_type = PolylineCurvedAndSmoothSurfaceType::CubicBSpline; // $SURFTYPE
        self.pedit_smooth_m_densith = 6; // $SURFU
        self.pedit_smooth_n_densith = 6; // $SURFV
        self.ucs_definition_name = String::new(); // $UCSBASE
        self.ucs_name = String::new(); // $UCSNAME
        self.ucs_origin = Point::origin(); // $UCSORG
        self.ucs_x_axis = Vector::x_axis(); // $UCSXDIR
        self.ucs_y_axis = Vector::y_axis(); // $UCSYDIR
        self.ortho_ucs_reference = String::new(); // $UCSORTHOREF
        self.orthgraphic_view_type = OrthographicViewType::None; // $UCSORTHOVIEW
        self.ucs_origin_top = Point::origin(); // $UCSORGTOP
        self.ucs_origin_bottom = Point::origin(); // $UCSORGBOTTOM
        self.ucs_origin_left = Point::origin(); // $UCSORGLEFT
        self.ucs_origin_right = Point::origin(); // $UCSORGRIGHT
        self.ucs_origin_front = Point::origin(); // $UCSORGFRONT
        self.ucs_origin_back = Point::origin(); // $UCSORGBACK
        self.paperspace_ucs_definition_name = String::new(); // $PUCSBASE
        self.paperspace_ucs_name = String::new(); // $PUCSNAME
        self.paperspace_ucs_origin = Point::origin(); // $PUCSORG
        self.paperspace_x_axis = Vector::x_axis(); // $PUCSXDIR
        self.paperspace_y_axis = Vector::y_axis(); // $PUCSYDIR
        self.paperspace_ortho_ucs_reference = String::new(); // $PUCSORTHOREF
        self.paperspace_orthographic_view_type = OrthographicViewType::None; // $PUCSORTHOVIEW
        self.paperspace_ucs_origin_top = Point::origin(); // $PUCSORGTOP
        self.paperspace_ucs_origin_bottom = Point::origin(); // $PUCSORGBOTTOM
        self.paperspace_ucs_origin_left = Point::origin(); // $PUCSORGLEFT
        self.paperspace_ucs_origin_right = Point::origin(); // $PUCSORGRIGHT
        self.paperspace_ucs_origin_front = Point::origin(); // $PUCSORGFRONT
        self.paperspace_ucs_origin_back = Point::origin(); // $PUCSORGBACK
        self.user_int1 = 0; // $USERI1
        self.user_int2 = 0; // $USERI2
        self.user_int3 = 0; // $USERI3
        self.user_int4 = 0; // $USERI4
        self.user_int5 = 0; // $USERI5
        self.user_real1 = 0.0; // $USERR1
        self.user_real2 = 0.0; // $USERR2
        self.user_real3 = 0.0; // $USERR3
        self.user_real4 = 0.0; // $USERR4
        self.user_real5 = 0.0; // $USERR5
        self.set_ucs_to_wcs_in_d_view_or_v_point = true; // $WORLDVIEW
        self.edge_shading = ShadeEdgeMode::FacesInEntityColorEdgesInBlack; // $SHADEDGE
        self.percent_ambient_to_diffuse = 70; // $SHADEDIF
        self.previous_release_tile_compatability = true; // $TILEMODE
        self.maximum_active_viewports = 64; // $MAXACTVP
        self.paperspace_insertion_base = Point::origin(); // $PINSBASE
        self.limit_checking_in_paperspace = false; // $PLIMCHECK
        self.paperspace_minimum_drawing_extents = Point::new(1.0e20, 1.0e20, 1.0e20); // $PEXTMIN
        self.paperspace_maximum_drawing_extents = Point::new(-1.0e20, -1.0e20, -1.0e20); // $PEXTMAX
        self.paperspace_minimum_drawing_limits = Point::origin(); // $PLIMMIN
        self.paperspace_maximum_drawing_limits = Point::new(12.0, 9.0, 0.0); // $PLIMMAX
        self.display_fractions_in_input = false; // $UNITMODE
        self.retain_x_ref_dependent_visibility_settings = true; // $VISRETAIN
        self.is_polyline_continuous_around_verticies = false; // $PLINEGEN
        self.scale_line_types_in_paperspace = true; // $PSLTSCALE
        self.spacial_index_max_depth = 3020; // $TREEDEPTH
        self.pick_style = PickStyle::Group; // $PICKSTYLE
        self.current_multiline_style = String::from("STANDARD"); // $CMLSTYLE
        self.current_multiline_justification = Justification::Top; // $CMLJUST
        self.current_multiline_scale = 1.0; // $CMLSCALE
        self.save_proxy_graphics = true; // $PROXYGRAPHICS
        self.drawing_units = DrawingUnits::English; // $MEASUREMENT
        self.new_object_line_weight = LineWeight::by_block(); // $CELWEIGHT
        self.end_cap_setting = EndCapSetting::None; // $ENDCAPS
        self.lineweight_joint_setting = JoinStyle::None; // $JOINSTYLE
        self.display_linewieght_in_model_and_layout_tab = false; // $LWDISPLAY
        self.default_drawing_units = Units::Unitless; // $INSUNITS
        self.hyperlink_base = String::new(); // $HYPERLINKBASE
        self.stylesheet = String::new(); // $STYLESHEET
        self.can_use_in_place_reference_editing = true; // $XEDIT
        self.new_object_plot_style_handle = Handle::empty(); // $CEPSNID
        self.new_object_plot_style = PlotStyle::ByLayer; // $CEPSNTYPE
        self.uses_color_dependent_plot_style_tables = true; // $PSTYLEMODE
        self.fingerprint_guid = Uuid::new_v4(); // $FINGERPRINTGUID
        self.version_guid = Uuid::new_v4(); // $VERSIONGUID
        self.use_acad2000_symbol_table_naming = true; // $EXTNAMES
        self.viewport_view_scale_factor = 0.0; // $PSVPSCALE
        self.ole_startup = false; // $OLESTARTUP
        self.object_sorting_methods_flags = 127; // $SORTENTS
        self.layer_and_spatial_index_save_mode = LayerAndSpatialIndexSaveMode::None; // $INDEXCTL
        self.hide_text_objects_when_producint_hidden_view = false; // $HIDETEXT
        self.is_x_ref_clipping_boundary_visible = XrefClippingBoundaryVisibility::DisplayedNotPlotted; // $XCLIPFRAME
        self.halo_gap_percent = 0.0; // $HALOGAP
        self.obscured_line_color = Color::by_entity(); // $OBSCOLOR
        self.obscured_line_type_style = LineTypeStyle::Off; // $OBSLTYPE
        self.display_intersection_polylines = false; // $INTERSECTIONDISPLAY
        self.intersection_polyline_color = Color::by_entity(); // $INTERSECTIONCOLOR
        self.dimension_object_associativity = DimensionAssociativity::NonAssociativeObjects; // $DIMASSOC
        self.project_name = String::new(); // $PROJECTNAME
        self.use_camera_display = false; // $CAMERADISPLAY
        self.lens_length = 50.0; // $LENSLENGTH
        self.camera_height = 0.0; // $CAMERAHEIGHT
        self.steps_per_second_in_walk_or_fly_mode = 2.0; // $STEPSPERSEC
        self.step_size_in_walk_or_fly_mode = 6.0; // $STEPSIZE
        self.dwf_3d_precision = Dwf3DPrecision::Deviation_0_5; // $3DDWFPREC
        self.last_poly_solid_width = 0.25; // $PSOLWIDTH
        self.last_poly_solid_height = 4.0; // $PSOLHEIGHT
        self.loft_operation_first_draft_angle = ::std::f64::consts::PI / 2.0; // $LOFTANG1
        self.loft_operation_second_draft_angle = ::std::f64::consts::PI / 2.0; // $LOFTANG2
        self.loft_operation_first_magnitude = 0.0; // $LOFTMAG1
        self.loft_operation_second_magnitude = 0.0; // $LOFTMAG2
        self.loft_flags = 7; // $LOFTPARAM
        self.lofted_object_normal_mode = LoftedObjectNormalMode::SmoothFit; // $LOFTNORMALS
        self.latitude = 37.7950; // $LATITUDE
        self.longitude = -122.3940; // $LONGITUDE
        self.angle_between_y_axis_and_north = 0.0; // $NORTHDIRECTION
        self.time_zone = DrawingTimeZone::PacificTime_US_Canada_SanFrancisco_Vancouver; // $TIMEZONE
        self.use_light_glyph_display = true; // $LIGHTGLYPHDISPLAY
        self.use_tile_mode_light_sync = true; // $TILEMODELIGHTSYNCH
        self.current_material_handle = Handle::empty(); // $CMATERIAL
        self.new_solids_contain_history = false; // $SOLIDHIST
        self.solid_history_mode = SolidHistoryMode::DoesNotOverride; // $SHOWHIST
        self.dwf_underlay_frame_mode = UnderlayFrameMode::DisplayNoPlot; // $DWFFRAME
        self.dgn_underlay_frame_mode = UnderlayFrameMode::None; // $DGNFRAME
        self.use_real_world_scale = true; // $REALWORLDSCALE
        self.interference_object_color = Color::from_index(1); // $INTERFERECOLOR
        self.interference_object_visual_style_pointer = Handle::empty(); // $INTERFEREOBJVS
        self.interference_view_port_visual_style_pointer = Handle::empty(); // $INTERFEREVPVS
        self.shadow_mode = ShadowMode::CastsAndReceivesShadows; // $CSHADOW
        self.shadow_plane_z_offset = 0.0; // $SHADOWPLANELOCATION
        self.axis_on = false; // $AXISMODE
        self.axis_tick_spacing = Vector::zero(); // $AXISUNIT
        self.fast_zoom = true; // $FASTZOOM
        self.grid_on = false; // $GRIDMODE
        self.grid_spacing = Vector::new(1.0, 1.0, 0.0); // $GRIDUNIT
        self.snap_rotation_angle = 0.0; // $SNAPANG
        self.snap_base_point = Point::origin(); // $SNAPBASE
        self.snap_isometric_plane = SnapIsometricPlane::Left; // $SNAPISOPAIR
        self.snap_on = false; // $SNAPMODE
        self.snap_style = SnapStyle::Standard; // $SNAPSTYLE
        self.snap_spacing = Vector::new(1.0, 1.0, 0.0); // $SNAPUNIT
        self.view_center = Point::origin(); // $VIEWCTR
        self.view_direction = Vector::z_axis(); // $VIEWDIR
        self.view_height = 1.0; // $VIEWSIZE
    }
    #[allow(clippy::cognitive_complexity)] // generated method
    pub(crate) fn set_header_value(&mut self, variable: &str, pair: &CodePair) -> DxfResult<()> {
        match variable {
            "$ACADVER" => { verify_code(pair, 1)?; self.version = AcadVersion::from_safe(pair.assert_string()?); },
            "$ACADMAINTVER" => {
                match pair.code {
                    70 => self.maintenance_version = pair.assert_i16()?,
                    90 => self.maintenance_version = pair.assert_i32()? as i16,
                    _ => return Err(DxfError::UnexpectedCodePair(pair.clone(), String::from("expected code [70, 90]"))),
                }
            },
            "$DWGCODEPAGE" => { verify_code(pair, 3)?; self.drawing_code_page = pair.assert_string()?; },
            "$LASTSAVEDBY" => { verify_code(pair, 1)?; self.last_saved_by = pair.assert_string()?; },
            "$REQUIREDVERSIONS" => { verify_code(pair, 160)?; self.required_versions = pair.assert_i64()?; },
            "$INSBASE" => { self.insertion_base.set(pair)?; },
            "$EXTMIN" => { self.minimum_drawing_extents.set(pair)?; },
            "$EXTMAX" => { self.maximum_drawing_extents.set(pair)?; },
            "$LIMMIN" => { self.minimum_drawing_limits.set(pair)?; },
            "$LIMMAX" => { self.maximum_drawing_limits.set(pair)?; },
            "$ORTHOMODE" => { verify_code(pair, 70)?; self.draw_orthogonal_lines = as_bool(pair.assert_i16()?); },
            "$REGENMODE" => { verify_code(pair, 70)?; self.use_regen_mode = as_bool(pair.assert_i16()?); },
            "$FILLMODE" => { verify_code(pair, 70)?; self.fill_mode_on = as_bool(pair.assert_i16()?); },
            "$QTEXTMODE" => { verify_code(pair, 70)?; self.use_quick_text_mode = as_bool(pair.assert_i16()?); },
            "$MIRRTEXT" => { verify_code(pair, 70)?; self.mirror_text = as_bool(pair.assert_i16()?); },
            "$DRAGMODE" => { verify_code(pair, 70)?; self.drag_mode = enum_from_number!(DragMode, Auto, from_i16, pair.assert_i16()?); },
            "$LTSCALE" => { verify_code(pair, 40)?; self.line_type_scale = pair.assert_f64()?; },
            "$OSMODE" => { verify_code(pair, 70)?; self.object_snap_flags = i32::from(pair.assert_i16()?); },
            "$ATTMODE" => { verify_code(pair, 70)?; self.attribute_visibility = enum_from_number!(AttributeVisibility, Normal, from_i16, pair.assert_i16()?); },
            "$TEXTSIZE" => { verify_code(pair, 40)?; self.default_text_height = pair.assert_f64()?; },
            "$TRACEWID" => { verify_code(pair, 40)?; self.trace_width = pair.assert_f64()?; },
            "$TEXTSTYLE" => { verify_code(pair, 7)?; self.text_style = pair.assert_string()?; },
            "$CLAYER" => { verify_code(pair, 8)?; self.current_layer = pair.assert_string()?; },
            "$CELTYPE" => { verify_code(pair, 6)?; self.current_entity_line_type = pair.assert_string()?; },
            "$CECOLOR" => { verify_code(pair, 62)?; self.current_entity_color = Color::from_raw_value(pair.assert_i16()?); },
            "$CELTSCALE" => { verify_code(pair, 40)?; self.current_entity_line_type_scale = pair.assert_f64()?; },
            "$DELOBJ" => { verify_code(pair, 70)?; self.retain_deleted_objects = as_bool(pair.assert_i16()?); },
            "$DISPSILH" => { verify_code(pair, 70)?; self.display_silhouette_curves_in_wireframe_mode = as_bool(pair.assert_i16()?); },
            "$DRAGVS" => { verify_code(pair, 349)?; self.solid_visual_style_pointer = pair.as_handle()?; },
            "$DIMSCALE" => { verify_code(pair, 40)?; self.dimensioning_scale_factor = pair.assert_f64()?; },
            "$DIMASZ" => { verify_code(pair, 40)?; self.dimensioning_arrow_size = pair.assert_f64()?; },
            "$DIMEXO" => { verify_code(pair, 40)?; self.dimension_extension_line_offset = pair.assert_f64()?; },
            "$DIMDLI" => { verify_code(pair, 40)?; self.dimension_line_increment = pair.assert_f64()?; },
            "$DIMRND" => { verify_code(pair, 40)?; self.dimension_distance_rounding_value = pair.assert_f64()?; },
            "$DIMDLE" => { verify_code(pair, 40)?; self.dimension_line_extension = pair.assert_f64()?; },
            "$DIMEXE" => { verify_code(pair, 40)?; self.dimension_extension_line_extension = pair.assert_f64()?; },
            "$DIMTP" => { verify_code(pair, 40)?; self.dimension_plus_tolerance = pair.assert_f64()?; },
            "$DIMTM" => { verify_code(pair, 40)?; self.dimension_minus_tolerance = pair.assert_f64()?; },
            "$DIMTXT" => { verify_code(pair, 40)?; self.dimensioning_text_height = pair.assert_f64()?; },
            "$DIMCEN" => { verify_code(pair, 40)?; self.center_mark_size = pair.assert_f64()?; },
            "$DIMTSZ" => { verify_code(pair, 40)?; self.dimensioning_tick_size = pair.assert_f64()?; },
            "$DIMTOL" => { verify_code(pair, 70)?; self.generate_dimension_tolerances = as_bool(pair.assert_i16()?); },
            "$DIMLIM" => { verify_code(pair, 70)?; self.generate_dimension_limits = as_bool(pair.assert_i16()?); },
            "$DIMTIH" => { verify_code(pair, 70)?; self.dimension_text_inside_horizontal = as_bool(pair.assert_i16()?); },
            "$DIMTOH" => { verify_code(pair, 70)?; self.dimension_text_outside_horizontal = as_bool(pair.assert_i16()?); },
            "$DIMSE1" => { verify_code(pair, 70)?; self.suppress_first_dimension_extension_line = as_bool(pair.assert_i16()?); },
            "$DIMSE2" => { verify_code(pair, 70)?; self.suppress_second_dimension_extension_line = as_bool(pair.assert_i16()?); },
            "$DIMTAD" => { verify_code(pair, 70)?; self.text_above_dimension_line = as_bool(pair.assert_i16()?); },
            "$DIMZIN" => { verify_code(pair, 70)?; self.dimension_unit_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
            "$DIMBLK" => { verify_code(pair, 1)?; self.arrow_block_name = pair.assert_string()?; },
            "$DIMASO" => { verify_code(pair, 70)?; self.create_associative_dimensioning = as_bool(pair.assert_i16()?); },
            "$DIMSHO" => { verify_code(pair, 70)?; self.recompute_dimensions_while_dragging = as_bool(pair.assert_i16()?); },
            "$DIMPOST" => { verify_code(pair, 1)?; self.dimensioning_suffix = pair.assert_string()?; },
            "$DIMAPOST" => { verify_code(pair, 1)?; self.alternate_dimensioning_suffix = pair.assert_string()?; },
            "$DIMALT" => { verify_code(pair, 70)?; self.use_alternate_dimensioning = as_bool(pair.assert_i16()?); },
            "$DIMALTD" => { verify_code(pair, 70)?; self.alternate_dimensioning_decimal_places = pair.assert_i16()?; },
            "$DIMALTF" => { verify_code(pair, 40)?; self.alternate_dimensioning_scale_factor = pair.assert_f64()?; },
            "$DIMLFAC" => { verify_code(pair, 40)?; self.dimension_linear_measurements_scale_factor = pair.assert_f64()?; },
            "$DIMTOFL" => { verify_code(pair, 70)?; self.force_dimension_line_extensions_outside_if_text_is = as_bool(pair.assert_i16()?); },
            "$DIMTVP" => { verify_code(pair, 40)?; self.dimension_vertical_text_position = pair.assert_f64()?; },
            "$DIMTIX" => { verify_code(pair, 70)?; self.force_dimension_text_inside_extensions = as_bool(pair.assert_i16()?); },
            "$DIMSOXD" => { verify_code(pair, 70)?; self.suppress_outside_extension_dimension_lines = as_bool(pair.assert_i16()?); },
            "$DIMSAH" => { verify_code(pair, 70)?; self.use_separate_arrow_blocks_for_dimensions = as_bool(pair.assert_i16()?); },
            "$DIMBLK1" => { verify_code(pair, 1)?; self.first_arrow_block_name = pair.assert_string()?; },
            "$DIMBLK2" => { verify_code(pair, 1)?; self.second_arrow_block_name = pair.assert_string()?; },
            "$DIMSTYLE" => { verify_code(pair, 2)?; self.dimension_style_name = pair.assert_string()?; },
            "$DIMCLRD" => { verify_code(pair, 70)?; self.dimension_line_color = Color::from_raw_value(pair.assert_i16()?); },
            "$DIMCLRE" => { verify_code(pair, 70)?; self.dimension_extension_line_color = Color::from_raw_value(pair.assert_i16()?); },
            "$DIMCLRT" => { verify_code(pair, 70)?; self.dimension_text_color = Color::from_raw_value(pair.assert_i16()?); },
            "$DIMTFAC" => { verify_code(pair, 40)?; self.dimension_tolerance_display_scale_factor = pair.assert_f64()?; },
            "$DIMGAP" => { verify_code(pair, 40)?; self.dimension_line_gap = pair.assert_f64()?; },
            "$DIMJUST" => { verify_code(pair, 70)?; self.dimension_text_justification = enum_from_number!(DimensionTextJustification, AboveLineCenter, from_i16, pair.assert_i16()?); },
            "$DIMTOLJ" => { verify_code(pair, 70)?; self.dimension_tolerance_vertical_justification = enum_from_number!(Justification, Middle, from_i16, pair.assert_i16()?); },
            "$DIMTZIN" => { verify_code(pair, 70)?; self.dimension_tolerance_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
            "$DIMALTZ" => { verify_code(pair, 70)?; self.alternate_dimensioning_zero_supression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
            "$DIMALTTZ" => { verify_code(pair, 70)?; self.alternate_dimensioning_tolerance_zero_supression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
            "$DIMFIT" => { verify_code(pair, 70)?; self.dimension_text_and_arrow_placement = enum_from_number!(DimensionFit, TextAndArrowsOutsideLines, from_i16, pair.assert_i16()?); },
            "$DIMUPT" => { verify_code(pair, 70)?; self.dimension_cursor_controls_text_position = as_bool(pair.assert_i16()?); },
            "$DIMUNIT" => { verify_code(pair, 70)?; self.dimension_unit_format = enum_from_number!(UnitFormat, Decimal, from_i16, pair.assert_i16()?); },
            "$DIMDEC" => { verify_code(pair, 70)?; self.dimension_unit_tolerance_decimal_places = pair.assert_i16()?; },
            "$DIMTDEC" => { verify_code(pair, 70)?; self.dimension_tolerance_decimal_places = pair.assert_i16()?; },
            "$DIMALTU" => { verify_code(pair, 70)?; self.alternate_dimensioning_units = enum_from_number!(UnitFormat, Decimal, from_i16, pair.assert_i16()?); },
            "$DIMALTTD" => { verify_code(pair, 70)?; self.alternate_dimensioning_tolerance_decimal_places = pair.assert_i16()?; },
            "$DIMTXSTY" => { verify_code(pair, 7)?; self.dimension_text_style = pair.assert_string()?; },
            "$DIMAUNIT" => { verify_code(pair, 70)?; self.dimensioning_angle_format = enum_from_number!(AngleFormat, DecimalDegrees, from_i16, pair.assert_i16()?); },
            "$DIMADEC" => { verify_code(pair, 70)?; self.angular_dimension_precision = pair.assert_i16()?; },
            "$DIMALTRND" => { verify_code(pair, 40)?; self.alternate_dimensioning_unit_rounding = pair.assert_f64()?; },
            "$DIMAZIN" => { verify_code(pair, 70)?; self.dimension_angle_zero_suppression = enum_from_number!(UnitZeroSuppression, SuppressZeroFeetAndZeroInches, from_i16, pair.assert_i16()?); },
            "$DIMDSEP" => { verify_code(pair, 70)?; self.dimension_decimal_separator_char = pair.assert_i16()? as u8 as char; },
            "$DIMFRAC" => { verify_code(pair, 70)?; self.dimension_text_height_scale_factor = enum_from_number!(DimensionFractionFormat, HorizontalStacking, from_i16, pair.assert_i16()?); },
            "$DIMLDRBLK" => { verify_code(pair, 1)?; self.dimension_leader_block_name = pair.assert_string()?; },
            "$DIMLUNIT" => { verify_code(pair, 70)?; self.dimension_non_angular_units = enum_from_number!(NonAngularUnits, Decimal, from_i16, pair.assert_i16()?); },
            "$DIMLWD" => { verify_code(pair, 70)?; self.dimension_line_weight = LineWeight::from_raw_value(pair.assert_i16()?); },
            "$DIMLWE" => { verify_code(pair, 70)?; self.dimension_extension_line_weight = LineWeight::from_raw_value(pair.assert_i16()?); },
            "$DIMTMOVE" => { verify_code(pair, 70)?; self.dimension_text_movement_rule = enum_from_number!(DimensionTextMovementRule, MoveLineWithText, from_i16, pair.assert_i16()?); },
            "$DIMFXL" => { verify_code(pair, 40)?; self.dimension_line_fixed_length = pair.assert_f64()?; },
            "$DIMFXLON" => { verify_code(pair, 70)?; self.dimension_line_fixed_length_on = as_bool(pair.assert_i16()?); },
            "$DIMJOGANG" => { verify_code(pair, 40)?; self.dimension_transverse_segment_angle_in_jogged_radius = pair.assert_f64()?; },
            "$DIMTFILL" => { verify_code(pair, 70)?; self.dimension_text_background_color_mode = enum_from_number!(DimensionTextBackgroundColorMode, None, from_i16, pair.assert_i16()?); },
            "$DIMTFILLCLR" => { verify_code(pair, 70)?; self.dxf_dimension_text_background_custom_color = Color::from_raw_value(pair.assert_i16()?); },
            "$DIMARCSYM" => { verify_code(pair, 70)?; self.dimension_arc_symbol_display_mode = enum_from_number!(DimensionArcSymbolDisplayMode, SymbolBeforeText, from_i16, pair.assert_i16()?); },
            "$DIMLTYPE" => { verify_code(pair, 6)?; self.dimension_line_type = pair.assert_string()?; },
            "$DIMLTEX1" => { verify_code(pair, 6)?; self.dimension_first_extension_line_type = pair.assert_string()?; },
            "$DIMLTEX2" => { verify_code(pair, 6)?; self.dimension_second_extension_line_type = pair.assert_string()?; },
            "$DIMTXTDIRECTION" => { verify_code(pair, 70)?; self.dimension_text_direction = enum_from_number!(TextDirection, LeftToRight, from_i16, pair.assert_i16()?); },
            "$LUNITS" => { verify_code(pair, 70)?; self.unit_format = enum_from_number!(UnitFormat, Decimal, from_i16, pair.assert_i16()?); },
            "$LUPREC" => { verify_code(pair, 70)?; self.unit_precision = pair.assert_i16()?; },
            "$SKETCHINC" => { verify_code(pair, 40)?; self.sketch_record_increment = pair.assert_f64()?; },
            "$FILLETRAD" => { verify_code(pair, 40)?; self.fillet_radius = pair.assert_f64()?; },
            "$AUNITS" => { verify_code(pair, 70)?; self.angle_unit_format = enum_from_number!(AngleFormat, DecimalDegrees, from_i16, pair.assert_i16()?); },
            "$AUPREC" => { verify_code(pair, 70)?; self.angle_unit_precision = pair.assert_i16()?; },
            "$MENU" => { verify_code(pair, 1)?; self.file_name = pair.assert_string()?; },
            "$ELEVATION" => { verify_code(pair, 40)?; self.elevation = pair.assert_f64()?; },
            "$PELEVATION" => { verify_code(pair, 40)?; self.paperspace_elevation = pair.assert_f64()?; },
            "$THICKNESS" => { verify_code(pair, 40)?; self.thickness = pair.assert_f64()?; },
            "$LIMCHECK" => { verify_code(pair, 70)?; self.use_limits_checking = as_bool(pair.assert_i16()?); },
            "$BLIPMODE" => { verify_code(pair, 70)?; self.blip_mode = as_bool(pair.assert_i16()?); },
            "$CHAMFERA" => { verify_code(pair, 40)?; self.first_chamfer_distance = pair.assert_f64()?; },
            "$CHAMFERB" => { verify_code(pair, 40)?; self.second_chamfer_distance = pair.assert_f64()?; },
            "$CHAMFERC" => { verify_code(pair, 40)?; self.chamfer_length = pair.assert_f64()?; },
            "$CHAMFERD" => { verify_code(pair, 40)?; self.chamfer_angle = pair.assert_f64()?; },
            "$SKPOLY" => { verify_code(pair, 70)?; self.polyline_sketch_mode = enum_from_number!(PolySketchMode, SketchLines, from_i16, pair.assert_i16()?); },
            "$TDCREATE" => { verify_code(pair, 40)?; self.creation_date = as_datetime_local(pair.assert_f64()?); },
            "$TDUCREATE" => { verify_code(pair, 40)?; self.creation_date_universal = as_datetime_utc(pair.assert_f64()?); },
            "$TDUPDATE" => { verify_code(pair, 40)?; self.update_date = as_datetime_local(pair.assert_f64()?); },
            "$TDUUPDATE" => { verify_code(pair, 40)?; self.update_date_universal = as_datetime_utc(pair.assert_f64()?); },
            "$TDINDWG" => { verify_code(pair, 40)?; self.time_in_drawing = as_duration(pair.assert_f64()?); },
            "$TDUSRTIMER" => { verify_code(pair, 40)?; self.user_elapsed_timer = as_duration(pair.assert_f64()?); },
            "$USRTIMER" => { verify_code(pair, 70)?; self.user_timer_on = as_bool(pair.assert_i16()?); },
            "$ANGBASE" => { verify_code(pair, 50)?; self.angle_zero_direction = pair.assert_f64()?; },
            "$ANGDIR" => { verify_code(pair, 70)?; self.angle_direction = enum_from_number!(AngleDirection, CounterClockwise, from_i16, pair.assert_i16()?); },
            "$PDMODE" => { verify_code(pair, 70)?; self.point_display_mode = i32::from(pair.assert_i16()?); },
            "$PDSIZE" => { verify_code(pair, 40)?; self.point_display_size = pair.assert_f64()?; },
            "$PLINEWID" => { verify_code(pair, 40)?; self.default_polyline_width = pair.assert_f64()?; },
            "$COORDS" => { verify_code(pair, 70)?; self.coordinate_display = enum_from_number!(CoordinateDisplay, ContinuousUpdate, from_i16, pair.assert_i16()?); },
            "$SPLFRAME" => { verify_code(pair, 70)?; self.display_spline_polygon_control = as_bool(pair.assert_i16()?); },
            "$SPLINETYPE" => { verify_code(pair, 70)?; self.pedit_spline_curve_type = enum_from_number!(PolylineCurvedAndSmoothSurfaceType, CubicBSpline, from_i16, pair.assert_i16()?); },
            "$SPLINESEGS" => { verify_code(pair, 70)?; self.line_segments_per_spline_patch = pair.assert_i16()?; },
            "$ATTDIA" => { verify_code(pair, 70)?; self.show_attribute_entry_dialogs = as_bool(pair.assert_i16()?); },
            "$ATTREQ" => { verify_code(pair, 70)?; self.prompt_for_attribute_on_insert = as_bool(pair.assert_i16()?); },
            "$HANDLING" => { verify_code(pair, 70)?; self.handles_enabled = as_bool(pair.assert_i16()?); },
            "$HANDSEED" => { verify_code(pair, 5)?; self.next_available_handle = pair.as_handle()?; },
            "$SURFTAB1" => { verify_code(pair, 70)?; self.mesh_tabulations_in_first_direction = pair.assert_i16()?; },
            "$SURFTAB2" => { verify_code(pair, 70)?; self.mesh_tabulations_in_second_direction = pair.assert_i16()?; },
            "$SURFTYPE" => { verify_code(pair, 70)?; self.pedit_smooth_surface_type = enum_from_number!(PolylineCurvedAndSmoothSurfaceType, CubicBSpline, from_i16, pair.assert_i16()?); },
            "$SURFU" => { verify_code(pair, 70)?; self.pedit_smooth_m_densith = pair.assert_i16()?; },
            "$SURFV" => { verify_code(pair, 70)?; self.pedit_smooth_n_densith = pair.assert_i16()?; },
            "$UCSBASE" => { verify_code(pair, 2)?; self.ucs_definition_name = pair.assert_string()?; },
            "$UCSNAME" => { verify_code(pair, 2)?; self.ucs_name = pair.assert_string()?; },
            "$UCSORG" => { self.ucs_origin.set(pair)?; },
            "$UCSXDIR" => { self.ucs_x_axis.set(pair)?; },
            "$UCSYDIR" => { self.ucs_y_axis.set(pair)?; },
            "$UCSORTHOREF" => { verify_code(pair, 2)?; self.ortho_ucs_reference = pair.assert_string()?; },
            "$UCSORTHOVIEW" => { verify_code(pair, 70)?; self.orthgraphic_view_type = enum_from_number!(OrthographicViewType, None, from_i16, pair.assert_i16()?); },
            "$UCSORGTOP" => { self.ucs_origin_top.set(pair)?; },
            "$UCSORGBOTTOM" => { self.ucs_origin_bottom.set(pair)?; },
            "$UCSORGLEFT" => { self.ucs_origin_left.set(pair)?; },
            "$UCSORGRIGHT" => { self.ucs_origin_right.set(pair)?; },
            "$UCSORGFRONT" => { self.ucs_origin_front.set(pair)?; },
            "$UCSORGBACK" => { self.ucs_origin_back.set(pair)?; },
            "$PUCSBASE" => { verify_code(pair, 2)?; self.paperspace_ucs_definition_name = pair.assert_string()?; },
            "$PUCSNAME" => { verify_code(pair, 2)?; self.paperspace_ucs_name = pair.assert_string()?; },
            "$PUCSORG" => { self.paperspace_ucs_origin.set(pair)?; },
            "$PUCSXDIR" => { self.paperspace_x_axis.set(pair)?; },
            "$PUCSYDIR" => { self.paperspace_y_axis.set(pair)?; },
            "$PUCSORTHOREF" => { verify_code(pair, 2)?; self.paperspace_ortho_ucs_reference = pair.assert_string()?; },
            "$PUCSORTHOVIEW" => { verify_code(pair, 70)?; self.paperspace_orthographic_view_type = enum_from_number!(OrthographicViewType, None, from_i16, pair.assert_i16()?); },
            "$PUCSORGTOP" => { self.paperspace_ucs_origin_top.set(pair)?; },
            "$PUCSORGBOTTOM" => { self.paperspace_ucs_origin_bottom.set(pair)?; },
            "$PUCSORGLEFT" => { self.paperspace_ucs_origin_left.set(pair)?; },
            "$PUCSORGRIGHT" => { self.paperspace_ucs_origin_right.set(pair)?; },
            "$PUCSORGFRONT" => { self.paperspace_ucs_origin_front.set(pair)?; },
            "$PUCSORGBACK" => { self.paperspace_ucs_origin_back.set(pair)?; },
            "$USERI1" => { verify_code(pair, 70)?; self.user_int1 = pair.assert_i16()?; },
            "$USERI2" => { verify_code(pair, 70)?; self.user_int2 = pair.assert_i16()?; },
            "$USERI3" => { verify_code(pair, 70)?; self.user_int3 = pair.assert_i16()?; },
            "$USERI4" => { verify_code(pair, 70)?; self.user_int4 = pair.assert_i16()?; },
            "$USERI5" => { verify_code(pair, 70)?; self.user_int5 = pair.assert_i16()?; },
            "$USERR1" => { verify_code(pair, 40)?; self.user_real1 = pair.assert_f64()?; },
            "$USERR2" => { verify_code(pair, 40)?; self.user_real2 = pair.assert_f64()?; },
            "$USERR3" => { verify_code(pair, 40)?; self.user_real3 = pair.assert_f64()?; },
            "$USERR4" => { verify_code(pair, 40)?; self.user_real4 = pair.assert_f64()?; },
            "$USERR5" => { verify_code(pair, 40)?; self.user_real5 = pair.assert_f64()?; },
            "$WORLDVIEW" => { verify_code(pair, 70)?; self.set_ucs_to_wcs_in_d_view_or_v_point = as_bool(pair.assert_i16()?); },
            "$SHADEDGE" => { verify_code(pair, 70)?; self.edge_shading = enum_from_number!(ShadeEdgeMode, FacesInEntityColorEdgesInBlack, from_i16, pair.assert_i16()?); },
            "$SHADEDIF" => { verify_code(pair, 70)?; self.percent_ambient_to_diffuse = pair.assert_i16()?; },
            "$TILEMODE" => { verify_code(pair, 70)?; self.previous_release_tile_compatability = as_bool(pair.assert_i16()?); },
            "$MAXACTVP" => { verify_code(pair, 70)?; self.maximum_active_viewports = pair.assert_i16()?; },
            "$PINSBASE" => { self.paperspace_insertion_base.set(pair)?; },
            "$PLIMCHECK" => { verify_code(pair, 70)?; self.limit_checking_in_paperspace = as_bool(pair.assert_i16()?); },
            "$PEXTMIN" => { self.paperspace_minimum_drawing_extents.set(pair)?; },
            "$PEXTMAX" => { self.paperspace_maximum_drawing_extents.set(pair)?; },
            "$PLIMMIN" => { self.paperspace_minimum_drawing_limits.set(pair)?; },
            "$PLIMMAX" => { self.paperspace_maximum_drawing_limits.set(pair)?; },
            "$UNITMODE" => { verify_code(pair, 70)?; self.display_fractions_in_input = as_bool(pair.assert_i16()?); },
            "$VISRETAIN" => { verify_code(pair, 70)?; self.retain_x_ref_dependent_visibility_settings = as_bool(pair.assert_i16()?); },
            "$PLINEGEN" => { verify_code(pair, 70)?; self.is_polyline_continuous_around_verticies = as_bool(pair.assert_i16()?); },
            "$PSLTSCALE" => { verify_code(pair, 70)?; self.scale_line_types_in_paperspace = as_bool(pair.assert_i16()?); },
            "$TREEDEPTH" => { verify_code(pair, 70)?; self.spacial_index_max_depth = pair.assert_i16()?; },
            "$PICKSTYLE" => { verify_code(pair, 70)?; self.pick_style = enum_from_number!(PickStyle, Group, from_i16, pair.assert_i16()?); },
            "$CMLSTYLE" => {
                match pair.code {
                    7 => self.current_multiline_style = pair.assert_string()?,
                    2 => self.current_multiline_style = pair.assert_string()?,
                    _ => return Err(DxfError::UnexpectedCodePair(pair.clone(), String::from("expected code [7, 2]"))),
                }
            },
            "$CMLJUST" => { verify_code(pair, 70)?; self.current_multiline_justification = enum_from_number!(Justification, Top, from_i16, pair.assert_i16()?); },
            "$CMLSCALE" => { verify_code(pair, 40)?; self.current_multiline_scale = pair.assert_f64()?; },
            "$PROXYGRAPHICS" => { verify_code(pair, 70)?; self.save_proxy_graphics = as_bool(pair.assert_i16()?); },
            "$MEASUREMENT" => { verify_code(pair, 70)?; self.drawing_units = enum_from_number!(DrawingUnits, English, from_i16, pair.assert_i16()?); },
            "$CELWEIGHT" => { verify_code(pair, 370)?; self.new_object_line_weight = LineWeight::from_raw_value(pair.assert_i16()?); },
            "$ENDCAPS" => { verify_code(pair, 280)?; self.end_cap_setting = enum_from_number!(EndCapSetting, None, from_i16, pair.assert_i16()?); },
            "$JOINSTYLE" => { verify_code(pair, 280)?; self.lineweight_joint_setting = enum_from_number!(JoinStyle, None, from_i16, pair.assert_i16()?); },
            "$LWDISPLAY" => { verify_code(pair, 290)?; self.display_linewieght_in_model_and_layout_tab = pair.assert_bool()?; },
            "$INSUNITS" => { verify_code(pair, 70)?; self.default_drawing_units = enum_from_number!(Units, Unitless, from_i16, pair.assert_i16()?); },
            "$HYPERLINKBASE" => { verify_code(pair, 1)?; self.hyperlink_base = pair.assert_string()?; },
            "$STYLESHEET" => { verify_code(pair, 1)?; self.stylesheet = pair.assert_string()?; },
            "$XEDIT" => { verify_code(pair, 290)?; self.can_use_in_place_reference_editing = pair.assert_bool()?; },
            "$CEPSNID" => { verify_code(pair, 390)?; self.new_object_plot_style_handle = pair.as_handle()?; },
            "$CEPSNTYPE" => { verify_code(pair, 380)?; self.new_object_plot_style = enum_from_number!(PlotStyle, ByLayer, from_i16, pair.assert_i16()?); },
            "$PSTYLEMODE" => { verify_code(pair, 290)?; self.uses_color_dependent_plot_style_tables = pair.assert_bool()?; },
            "$FINGERPRINTGUID" => { verify_code(pair, 2)?; self.fingerprint_guid = as_uuid(pair.assert_string()?); },
            "$VERSIONGUID" => { verify_code(pair, 2)?; self.version_guid = as_uuid(pair.assert_string()?); },
            "$EXTNAMES" => { verify_code(pair, 290)?; self.use_acad2000_symbol_table_naming = pair.assert_bool()?; },
            "$PSVPSCALE" => { verify_code(pair, 40)?; self.viewport_view_scale_factor = pair.assert_f64()?; },
            "$OLESTARTUP" => { verify_code(pair, 290)?; self.ole_startup = pair.assert_bool()?; },
            "$SORTENTS" => { verify_code(pair, 280)?; self.object_sorting_methods_flags = i32::from(pair.assert_i16()?); },
            "$INDEXCTL" => { verify_code(pair, 280)?; self.layer_and_spatial_index_save_mode = enum_from_number!(LayerAndSpatialIndexSaveMode, None, from_i16, pair.assert_i16()?); },
            "$HIDETEXT" => {
                match pair.code {
                    280 => self.hide_text_objects_when_producint_hidden_view = as_bool(pair.assert_i16()?),
                    290 => self.hide_text_objects_when_producint_hidden_view = pair.assert_bool()?,
                    _ => return Err(DxfError::UnexpectedCodePair(pair.clone(), String::from("expected code [280, 290]"))),
                }
            },
            "$XCLIPFRAME" => {
                match pair.code {
                    290 => self.is_x_ref_clipping_boundary_visible = clipping_from_bool(pair.assert_bool()?),
                    280 => self.is_x_ref_clipping_boundary_visible = enum_from_number!(XrefClippingBoundaryVisibility, DisplayedNotPlotted, from_i16, pair.assert_i16()?),
                    _ => return Err(DxfError::UnexpectedCodePair(pair.clone(), String::from("expected code [290, 280]"))),
                }
            },
            "$HALOGAP" => { verify_code(pair, 280)?; self.halo_gap_percent = f64::from(pair.assert_i16()?); },
            "$OBSCOLOR" => { verify_code(pair, 70)?; self.obscured_line_color = Color::from_raw_value(pair.assert_i16()?); },
            "$OBSLTYPE" => { verify_code(pair, 280)?; self.obscured_line_type_style = enum_from_number!(LineTypeStyle, Off, from_i16, pair.assert_i16()?); },
            "$INTERSECTIONDISPLAY" => {
                match pair.code {
                    280 => self.display_intersection_polylines = as_bool(pair.assert_i16()?),
                    290 => self.display_intersection_polylines = pair.assert_bool()?,
                    _ => return Err(DxfError::UnexpectedCodePair(pair.clone(), String::from("expected code [280, 290]"))),
                }
            },
            "$INTERSECTIONCOLOR" => { verify_code(pair, 70)?; self.intersection_polyline_color = Color::from_raw_value(pair.assert_i16()?); },
            "$DIMASSOC" => { verify_code(pair, 280)?; self.dimension_object_associativity = enum_from_number!(DimensionAssociativity, NonAssociativeObjects, from_i16, pair.assert_i16()?); },
            "$PROJECTNAME" => { verify_code(pair, 1)?; self.project_name = pair.assert_string()?; },
            "$CAMERADISPLAY" => { verify_code(pair, 290)?; self.use_camera_display = pair.assert_bool()?; },
            "$LENSLENGTH" => { verify_code(pair, 40)?; self.lens_length = pair.assert_f64()?; },
            "$CAMERAHEIGHT" => { verify_code(pair, 40)?; self.camera_height = pair.assert_f64()?; },
            "$STEPSPERSEC" => { verify_code(pair, 40)?; self.steps_per_second_in_walk_or_fly_mode = pair.assert_f64()?; },
            "$STEPSIZE" => { verify_code(pair, 40)?; self.step_size_in_walk_or_fly_mode = pair.assert_f64()?; },
            "$3DDWFPREC" => { verify_code(pair, 40)?; self.dwf_3d_precision = enum_from_number!(Dwf3DPrecision, Deviation_0_5, from_f64, pair.assert_f64()?); },
            "$PSOLWIDTH" => { verify_code(pair, 40)?; self.last_poly_solid_width = pair.assert_f64()?; },
            "$PSOLHEIGHT" => { verify_code(pair, 40)?; self.last_poly_solid_height = pair.assert_f64()?; },
            "$LOFTANG1" => { verify_code(pair, 40)?; self.loft_operation_first_draft_angle = pair.assert_f64()?; },
            "$LOFTANG2" => { verify_code(pair, 40)?; self.loft_operation_second_draft_angle = pair.assert_f64()?; },
            "$LOFTMAG1" => { verify_code(pair, 40)?; self.loft_operation_first_magnitude = pair.assert_f64()?; },
            "$LOFTMAG2" => { verify_code(pair, 40)?; self.loft_operation_second_magnitude = pair.assert_f64()?; },
            "$LOFTPARAM" => { verify_code(pair, 70)?; self.loft_flags = i32::from(pair.assert_i16()?); },
            "$LOFTNORMALS" => { verify_code(pair, 280)?; self.lofted_object_normal_mode = enum_from_number!(LoftedObjectNormalMode, SmoothFit, from_i16, pair.assert_i16()?); },
            "$LATITUDE" => { verify_code(pair, 40)?; self.latitude = pair.assert_f64()?; },
            "$LONGITUDE" => { verify_code(pair, 40)?; self.longitude = pair.assert_f64()?; },
            "$NORTHDIRECTION" => { verify_code(pair, 40)?; self.angle_between_y_axis_and_north = pair.assert_f64()?; },
            "$TIMEZONE" => { verify_code(pair, 70)?; self.time_zone = enum_from_number!(DrawingTimeZone, PacificTime_US_Canada_SanFrancisco_Vancouver, from_i16, pair.assert_i16()?); },
            "$LIGHTGLYPHDISPLAY" => { verify_code(pair, 280)?; self.use_light_glyph_display = as_bool(pair.assert_i16()?); },
            "$TILEMODELIGHTSYNCH" => { verify_code(pair, 280)?; self.use_tile_mode_light_sync = as_bool(pair.assert_i16()?); },
            "$CMATERIAL" => { verify_code(pair, 347)?; self.current_material_handle = pair.as_handle()?; },
            "$SOLIDHIST" => { verify_code(pair, 280)?; self.new_solids_contain_history = as_bool(pair.assert_i16()?); },
            "$SHOWHIST" => { verify_code(pair, 280)?; self.solid_history_mode = enum_from_number!(SolidHistoryMode, DoesNotOverride, from_i16, pair.assert_i16()?); },
            "$DWFFRAME" => { verify_code(pair, 280)?; self.dwf_underlay_frame_mode = enum_from_number!(UnderlayFrameMode, DisplayNoPlot, from_i16, pair.assert_i16()?); },
            "$DGNFRAME" => { verify_code(pair, 280)?; self.dgn_underlay_frame_mode = enum_from_number!(UnderlayFrameMode, None, from_i16, pair.assert_i16()?); },
            "$REALWORLDSCALE" => { verify_code(pair, 290)?; self.use_real_world_scale = pair.assert_bool()?; },
            "$INTERFERECOLOR" => { verify_code(pair, 62)?; self.interference_object_color = Color::from_raw_value(pair.assert_i16()?); },
            "$INTERFEREOBJVS" => { verify_code(pair, 345)?; self.interference_object_visual_style_pointer = pair.as_handle()?; },
            "$INTERFEREVPVS" => { verify_code(pair, 346)?; self.interference_view_port_visual_style_pointer = pair.as_handle()?; },
            "$CSHADOW" => { verify_code(pair, 280)?; self.shadow_mode = enum_from_number!(ShadowMode, CastsAndReceivesShadows, from_i16, pair.assert_i16()?); },
            "$SHADOWPLANELOCATION" => { verify_code(pair, 40)?; self.shadow_plane_z_offset = pair.assert_f64()?; },
            "$AXISMODE" => { verify_code(pair, 70)?; self.axis_on = as_bool(pair.assert_i16()?); },
            "$AXISUNIT" => { self.axis_tick_spacing.set(pair)?; },
            "$FASTZOOM" => { verify_code(pair, 70)?; self.fast_zoom = as_bool(pair.assert_i16()?); },
            "$GRIDMODE" => { verify_code(pair, 70)?; self.grid_on = as_bool(pair.assert_i16()?); },
            "$GRIDUNIT" => { self.grid_spacing.set(pair)?; },
            "$SNAPANG" => { verify_code(pair, 50)?; self.snap_rotation_angle = pair.assert_f64()?; },
            "$SNAPBASE" => { self.snap_base_point.set(pair)?; },
            "$SNAPISOPAIR" => { verify_code(pair, 70)?; self.snap_isometric_plane = enum_from_number!(SnapIsometricPlane, Left, from_i16, pair.assert_i16()?); },
            "$SNAPMODE" => { verify_code(pair, 70)?; self.snap_on = as_bool(pair.assert_i16()?); },
            "$SNAPSTYLE" => { verify_code(pair, 70)?; self.snap_style = enum_from_number!(SnapStyle, Standard, from_i16, pair.assert_i16()?); },
            "$SNAPUNIT" => { self.snap_spacing.set(pair)?; },
            "$VIEWCTR" => { self.view_center.set(pair)?; },
            "$VIEWDIR" => { self.view_direction.set(pair)?; },
            "$VIEWSIZE" => { verify_code(pair, 40)?; self.view_height = pair.assert_f64()?; },
            _ => (),
        }

        Ok(())
    }
    #[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
    pub(crate) fn add_code_pairs_internal(&self, pairs: &mut Vec<CodePair>) {
        // $ACADVER
        pairs.push(CodePair::new_str(9, "$ACADVER"));
        pairs.push(CodePair::new_string(1, &self.version.to_string()));

        // $ACADMAINTVER
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$ACADMAINTVER"));
            pairs.push(CodePair::new_i16(70, self.maintenance_version));
        }

        // $DWGCODEPAGE
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DWGCODEPAGE"));
            pairs.push(CodePair::new_string(3, &self.drawing_code_page));
        }

        // $LASTSAVEDBY
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$LASTSAVEDBY"));
            pairs.push(CodePair::new_string(1, &self.last_saved_by));
        }

        // $REQUIREDVERSIONS
        if self.version >= AcadVersion::R2013 {
            pairs.push(CodePair::new_str(9, "$REQUIREDVERSIONS"));
            pairs.push(CodePair::new_i64(160, self.required_versions));
        }

        // $INSBASE
        pairs.push(CodePair::new_str(9, "$INSBASE"));
        pairs.push(CodePair::new_f64(10, self.insertion_base.x));
        pairs.push(CodePair::new_f64(20, self.insertion_base.y));
        pairs.push(CodePair::new_f64(30, self.insertion_base.z));

        // $EXTMIN
        pairs.push(CodePair::new_str(9, "$EXTMIN"));
        pairs.push(CodePair::new_f64(10, self.minimum_drawing_extents.x));
        pairs.push(CodePair::new_f64(20, self.minimum_drawing_extents.y));
        pairs.push(CodePair::new_f64(30, self.minimum_drawing_extents.z));

        // $EXTMAX
        pairs.push(CodePair::new_str(9, "$EXTMAX"));
        pairs.push(CodePair::new_f64(10, self.maximum_drawing_extents.x));
        pairs.push(CodePair::new_f64(20, self.maximum_drawing_extents.y));
        pairs.push(CodePair::new_f64(30, self.maximum_drawing_extents.z));

        // $LIMMIN
        pairs.push(CodePair::new_str(9, "$LIMMIN"));
        pairs.push(CodePair::new_f64(10, self.minimum_drawing_limits.x));
        pairs.push(CodePair::new_f64(20, self.minimum_drawing_limits.y));

        // $LIMMAX
        pairs.push(CodePair::new_str(9, "$LIMMAX"));
        pairs.push(CodePair::new_f64(10, self.maximum_drawing_limits.x));
        pairs.push(CodePair::new_f64(20, self.maximum_drawing_limits.y));

        // $ORTHOMODE
        pairs.push(CodePair::new_str(9, "$ORTHOMODE"));
        pairs.push(CodePair::new_i16(70, as_i16(self.draw_orthogonal_lines)));

        // $REGENMODE
        pairs.push(CodePair::new_str(9, "$REGENMODE"));
        pairs.push(CodePair::new_i16(70, as_i16(self.use_regen_mode)));

        // $FILLMODE
        pairs.push(CodePair::new_str(9, "$FILLMODE"));
        pairs.push(CodePair::new_i16(70, as_i16(self.fill_mode_on)));

        // $QTEXTMODE
        pairs.push(CodePair::new_str(9, "$QTEXTMODE"));
        pairs.push(CodePair::new_i16(70, as_i16(self.use_quick_text_mode)));

        // $MIRRTEXT
        pairs.push(CodePair::new_str(9, "$MIRRTEXT"));
        pairs.push(CodePair::new_i16(70, as_i16(self.mirror_text)));

        // $DRAGMODE
        if self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$DRAGMODE"));
            pairs.push(CodePair::new_i16(70, self.drag_mode as i16));
        }

        // $LTSCALE
        pairs.push(CodePair::new_str(9, "$LTSCALE"));
        pairs.push(CodePair::new_f64(40, self.line_type_scale));

        // $OSMODE
        if self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$OSMODE"));
            pairs.push(CodePair::new_i16(70, self.object_snap_flags as i16));
        }

        // $ATTMODE
        pairs.push(CodePair::new_str(9, "$ATTMODE"));
        pairs.push(CodePair::new_i16(70, self.attribute_visibility as i16));

        // $TEXTSIZE
        pairs.push(CodePair::new_str(9, "$TEXTSIZE"));
        pairs.push(CodePair::new_f64(40, self.default_text_height));

        // $TRACEWID
        pairs.push(CodePair::new_str(9, "$TRACEWID"));
        pairs.push(CodePair::new_f64(40, self.trace_width));

        // $TEXTSTYLE
        pairs.push(CodePair::new_str(9, "$TEXTSTYLE"));
        pairs.push(CodePair::new_string(7, &self.text_style));

        // $CLAYER
        pairs.push(CodePair::new_str(9, "$CLAYER"));
        pairs.push(CodePair::new_string(8, &self.current_layer));

        // $CELTYPE
        pairs.push(CodePair::new_str(9, "$CELTYPE"));
        pairs.push(CodePair::new_string(6, &self.current_entity_line_type));

        // $CECOLOR
        pairs.push(CodePair::new_str(9, "$CECOLOR"));
        pairs.push(CodePair::new_i16(62, self.current_entity_color.raw_value()));

        // $CELTSCALE
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$CELTSCALE"));
            pairs.push(CodePair::new_f64(40, self.current_entity_line_type_scale));
        }

        // $DELOBJ
        if self.version >= AcadVersion::R13 && self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$DELOBJ"));
            pairs.push(CodePair::new_i16(70, as_i16(self.retain_deleted_objects)));
        }

        // $DISPSILH
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DISPSILH"));
            pairs.push(CodePair::new_i16(70, as_i16(self.display_silhouette_curves_in_wireframe_mode)));
        }

        // $DRAGVS
        if self.version >= AcadVersion::R2007 && self.solid_visual_style_pointer != Handle::empty() {
            pairs.push(CodePair::new_str(9, "$DRAGVS"));
            pairs.push(CodePair::new_string(349, &self.solid_visual_style_pointer.as_string()));
        }

        // $DIMSCALE
        pairs.push(CodePair::new_str(9, "$DIMSCALE"));
        pairs.push(CodePair::new_f64(40, self.dimensioning_scale_factor));

        // $DIMASZ
        pairs.push(CodePair::new_str(9, "$DIMASZ"));
        pairs.push(CodePair::new_f64(40, self.dimensioning_arrow_size));

        // $DIMEXO
        pairs.push(CodePair::new_str(9, "$DIMEXO"));
        pairs.push(CodePair::new_f64(40, self.dimension_extension_line_offset));

        // $DIMDLI
        pairs.push(CodePair::new_str(9, "$DIMDLI"));
        pairs.push(CodePair::new_f64(40, self.dimension_line_increment));

        // $DIMRND
        pairs.push(CodePair::new_str(9, "$DIMRND"));
        pairs.push(CodePair::new_f64(40, self.dimension_distance_rounding_value));

        // $DIMDLE
        pairs.push(CodePair::new_str(9, "$DIMDLE"));
        pairs.push(CodePair::new_f64(40, self.dimension_line_extension));

        // $DIMEXE
        pairs.push(CodePair::new_str(9, "$DIMEXE"));
        pairs.push(CodePair::new_f64(40, self.dimension_extension_line_extension));

        // $DIMTP
        pairs.push(CodePair::new_str(9, "$DIMTP"));
        pairs.push(CodePair::new_f64(40, self.dimension_plus_tolerance));

        // $DIMTM
        pairs.push(CodePair::new_str(9, "$DIMTM"));
        pairs.push(CodePair::new_f64(40, self.dimension_minus_tolerance));

        // $DIMTXT
        pairs.push(CodePair::new_str(9, "$DIMTXT"));
        pairs.push(CodePair::new_f64(40, self.dimensioning_text_height));

        // $DIMCEN
        pairs.push(CodePair::new_str(9, "$DIMCEN"));
        pairs.push(CodePair::new_f64(40, self.center_mark_size));

        // $DIMTSZ
        pairs.push(CodePair::new_str(9, "$DIMTSZ"));
        pairs.push(CodePair::new_f64(40, self.dimensioning_tick_size));

        // $DIMTOL
        pairs.push(CodePair::new_str(9, "$DIMTOL"));
        pairs.push(CodePair::new_i16(70, as_i16(self.generate_dimension_tolerances)));

        // $DIMLIM
        pairs.push(CodePair::new_str(9, "$DIMLIM"));
        pairs.push(CodePair::new_i16(70, as_i16(self.generate_dimension_limits)));

        // $DIMTIH
        pairs.push(CodePair::new_str(9, "$DIMTIH"));
        pairs.push(CodePair::new_i16(70, as_i16(self.dimension_text_inside_horizontal)));

        // $DIMTOH
        pairs.push(CodePair::new_str(9, "$DIMTOH"));
        pairs.push(CodePair::new_i16(70, as_i16(self.dimension_text_outside_horizontal)));

        // $DIMSE1
        if self.version >= AcadVersion::R12 {
            pairs.push(CodePair::new_str(9, "$DIMSE1"));
            pairs.push(CodePair::new_i16(70, as_i16(self.suppress_first_dimension_extension_line)));
        }

        // $DIMSE2
        if self.version >= AcadVersion::R12 {
            pairs.push(CodePair::new_str(9, "$DIMSE2"));
            pairs.push(CodePair::new_i16(70, as_i16(self.suppress_second_dimension_extension_line)));
        }

        // $DIMTAD
        pairs.push(CodePair::new_str(9, "$DIMTAD"));
        pairs.push(CodePair::new_i16(70, as_i16(self.text_above_dimension_line)));

        // $DIMZIN
        pairs.push(CodePair::new_str(9, "$DIMZIN"));
        pairs.push(CodePair::new_i16(70, self.dimension_unit_zero_suppression as i16));

        // $DIMBLK
        pairs.push(CodePair::new_str(9, "$DIMBLK"));
        pairs.push(CodePair::new_string(1, &self.arrow_block_name));

        // $DIMASO
        pairs.push(CodePair::new_str(9, "$DIMASO"));
        pairs.push(CodePair::new_i16(70, as_i16(self.create_associative_dimensioning)));

        // $DIMSHO
        pairs.push(CodePair::new_str(9, "$DIMSHO"));
        pairs.push(CodePair::new_i16(70, as_i16(self.recompute_dimensions_while_dragging)));

        // $DIMPOST
        pairs.push(CodePair::new_str(9, "$DIMPOST"));
        pairs.push(CodePair::new_string(1, &self.dimensioning_suffix));

        // $DIMAPOST
        pairs.push(CodePair::new_str(9, "$DIMAPOST"));
        pairs.push(CodePair::new_string(1, &self.alternate_dimensioning_suffix));

        // $DIMALT
        pairs.push(CodePair::new_str(9, "$DIMALT"));
        pairs.push(CodePair::new_i16(70, as_i16(self.use_alternate_dimensioning)));

        // $DIMALTD
        pairs.push(CodePair::new_str(9, "$DIMALTD"));
        pairs.push(CodePair::new_i16(70, self.alternate_dimensioning_decimal_places));

        // $DIMALTF
        pairs.push(CodePair::new_str(9, "$DIMALTF"));
        pairs.push(CodePair::new_f64(40, self.alternate_dimensioning_scale_factor));

        // $DIMLFAC
        pairs.push(CodePair::new_str(9, "$DIMLFAC"));
        pairs.push(CodePair::new_f64(40, self.dimension_linear_measurements_scale_factor));

        // $DIMTOFL
        pairs.push(CodePair::new_str(9, "$DIMTOFL"));
        pairs.push(CodePair::new_i16(70, as_i16(self.force_dimension_line_extensions_outside_if_text_is)));

        // $DIMTVP
        pairs.push(CodePair::new_str(9, "$DIMTVP"));
        pairs.push(CodePair::new_f64(40, self.dimension_vertical_text_position));

        // $DIMTIX
        pairs.push(CodePair::new_str(9, "$DIMTIX"));
        pairs.push(CodePair::new_i16(70, as_i16(self.force_dimension_text_inside_extensions)));

        // $DIMSOXD
        pairs.push(CodePair::new_str(9, "$DIMSOXD"));
        pairs.push(CodePair::new_i16(70, as_i16(self.suppress_outside_extension_dimension_lines)));

        // $DIMSAH
        pairs.push(CodePair::new_str(9, "$DIMSAH"));
        pairs.push(CodePair::new_i16(70, as_i16(self.use_separate_arrow_blocks_for_dimensions)));

        // $DIMBLK1
        pairs.push(CodePair::new_str(9, "$DIMBLK1"));
        pairs.push(CodePair::new_string(1, &self.first_arrow_block_name));

        // $DIMBLK2
        pairs.push(CodePair::new_str(9, "$DIMBLK2"));
        pairs.push(CodePair::new_string(1, &self.second_arrow_block_name));

        // $DIMSTYLE
        pairs.push(CodePair::new_str(9, "$DIMSTYLE"));
        pairs.push(CodePair::new_string(2, &self.dimension_style_name));

        // $DIMCLRD
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$DIMCLRD"));
            pairs.push(CodePair::new_i16(70, self.dimension_line_color.raw_value()));
        }

        // $DIMCLRE
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$DIMCLRE"));
            pairs.push(CodePair::new_i16(70, self.dimension_extension_line_color.raw_value()));
        }

        // $DIMCLRT
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$DIMCLRT"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_color.raw_value()));
        }

        // $DIMTFAC
        if self.version >= AcadVersion::R12 {
            pairs.push(CodePair::new_str(9, "$DIMTFAC"));
            pairs.push(CodePair::new_f64(40, self.dimension_tolerance_display_scale_factor));
        }

        // $DIMGAP
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$DIMGAP"));
            pairs.push(CodePair::new_f64(40, self.dimension_line_gap));
        }

        // $DIMJUST
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMJUST"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_justification as i16));
        }

        // $DIMSD1
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMSD1"));
            pairs.push(CodePair::new_i16(70, as_i16(self.suppress_first_dimension_extension_line)));
        }

        // $DIMSD2
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMSD2"));
            pairs.push(CodePair::new_i16(70, as_i16(self.suppress_second_dimension_extension_line)));
        }

        // $DIMTOLJ
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMTOLJ"));
            pairs.push(CodePair::new_i16(70, self.dimension_tolerance_vertical_justification as i16));
        }

        // $DIMTZIN
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMTZIN"));
            pairs.push(CodePair::new_i16(70, self.dimension_tolerance_zero_suppression as i16));
        }

        // $DIMALTZ
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMALTZ"));
            pairs.push(CodePair::new_i16(70, self.alternate_dimensioning_zero_supression as i16));
        }

        // $DIMALTTZ
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMALTTZ"));
            pairs.push(CodePair::new_i16(70, self.alternate_dimensioning_tolerance_zero_supression as i16));
        }

        // $DIMFIT
        if self.version >= AcadVersion::R13 && self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$DIMFIT"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_and_arrow_placement as i16));
        }

        // $DIMUPT
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMUPT"));
            pairs.push(CodePair::new_i16(70, as_i16(self.dimension_cursor_controls_text_position)));
        }

        // $DIMUNIT
        if self.version >= AcadVersion::R13 && self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$DIMUNIT"));
            pairs.push(CodePair::new_i16(70, self.dimension_unit_format as i16));
        }

        // $DIMDEC
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMDEC"));
            pairs.push(CodePair::new_i16(70, self.dimension_unit_tolerance_decimal_places));
        }

        // $DIMTDEC
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMTDEC"));
            pairs.push(CodePair::new_i16(70, self.dimension_tolerance_decimal_places));
        }

        // $DIMALTU
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMALTU"));
            pairs.push(CodePair::new_i16(70, self.alternate_dimensioning_units as i16));
        }

        // $DIMALTTD
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMALTTD"));
            pairs.push(CodePair::new_i16(70, self.alternate_dimensioning_tolerance_decimal_places));
        }

        // $DIMTXSTY
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMTXSTY"));
            pairs.push(CodePair::new_string(7, &self.dimension_text_style));
        }

        // $DIMAUNIT
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$DIMAUNIT"));
            pairs.push(CodePair::new_i16(70, self.dimensioning_angle_format as i16));
        }

        // $DIMADEC
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMADEC"));
            pairs.push(CodePair::new_i16(70, self.angular_dimension_precision));
        }

        // $DIMALTRND
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMALTRND"));
            pairs.push(CodePair::new_f64(40, self.alternate_dimensioning_unit_rounding));
        }

        // $DIMAZIN
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMAZIN"));
            pairs.push(CodePair::new_i16(70, self.dimension_angle_zero_suppression as i16));
        }

        // $DIMDSEP
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMDSEP"));
            pairs.push(CodePair::new_i16(70, self.dimension_decimal_separator_char as i16));
        }

        // $DIMATFIT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMATFIT"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_and_arrow_placement as i16));
        }

        // $DIMFRAC
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMFRAC"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_height_scale_factor as i16));
        }

        // $DIMLDRBLK
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMLDRBLK"));
            pairs.push(CodePair::new_string(1, &self.dimension_leader_block_name));
        }

        // $DIMLUNIT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMLUNIT"));
            pairs.push(CodePair::new_i16(70, self.dimension_non_angular_units as i16));
        }

        // $DIMLWD
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMLWD"));
            pairs.push(CodePair::new_i16(70, self.dimension_line_weight.raw_value()));
        }

        // $DIMLWE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMLWE"));
            pairs.push(CodePair::new_i16(70, self.dimension_extension_line_weight.raw_value()));
        }

        // $DIMTMOVE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$DIMTMOVE"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_movement_rule as i16));
        }

        // $DIMFXL
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMFXL"));
            pairs.push(CodePair::new_f64(40, self.dimension_line_fixed_length));
        }

        // $DIMFXLON
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMFXLON"));
            pairs.push(CodePair::new_i16(70, as_i16(self.dimension_line_fixed_length_on)));
        }

        // $DIMJOGANG
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMJOGANG"));
            pairs.push(CodePair::new_f64(40, self.dimension_transverse_segment_angle_in_jogged_radius));
        }

        // $DIMTFILL
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMTFILL"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_background_color_mode as i16));
        }

        // $DIMTFILLCLR
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMTFILLCLR"));
            pairs.push(CodePair::new_i16(70, self.dxf_dimension_text_background_custom_color.raw_value()));
        }

        // $DIMARCSYM
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMARCSYM"));
            pairs.push(CodePair::new_i16(70, self.dimension_arc_symbol_display_mode as i16));
        }

        // $DIMLTYPE
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMLTYPE"));
            pairs.push(CodePair::new_string(6, &self.dimension_line_type));
        }

        // $DIMLTEX1
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMLTEX1"));
            pairs.push(CodePair::new_string(6, &self.dimension_first_extension_line_type));
        }

        // $DIMLTEX2
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DIMLTEX2"));
            pairs.push(CodePair::new_string(6, &self.dimension_second_extension_line_type));
        }

        // $DIMTXTDIRECTION
        if self.version >= AcadVersion::R2010 {
            pairs.push(CodePair::new_str(9, "$DIMTXTDIRECTION"));
            pairs.push(CodePair::new_i16(70, self.dimension_text_direction as i16));
        }

        // $LUNITS
        pairs.push(CodePair::new_str(9, "$LUNITS"));
        pairs.push(CodePair::new_i16(70, self.unit_format as i16));

        // $LUPREC
        pairs.push(CodePair::new_str(9, "$LUPREC"));
        pairs.push(CodePair::new_i16(70, self.unit_precision));

        // $SKETCHINC
        pairs.push(CodePair::new_str(9, "$SKETCHINC"));
        pairs.push(CodePair::new_f64(40, self.sketch_record_increment));

        // $FILLETRAD
        pairs.push(CodePair::new_str(9, "$FILLETRAD"));
        pairs.push(CodePair::new_f64(40, self.fillet_radius));

        // $AUNITS
        pairs.push(CodePair::new_str(9, "$AUNITS"));
        pairs.push(CodePair::new_i16(70, self.angle_unit_format as i16));

        // $AUPREC
        pairs.push(CodePair::new_str(9, "$AUPREC"));
        pairs.push(CodePair::new_i16(70, self.angle_unit_precision));

        // $MENU
        pairs.push(CodePair::new_str(9, "$MENU"));
        pairs.push(CodePair::new_string(1, &self.file_name));

        // $ELEVATION
        pairs.push(CodePair::new_str(9, "$ELEVATION"));
        pairs.push(CodePair::new_f64(40, self.elevation));

        // $PELEVATION
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PELEVATION"));
            pairs.push(CodePair::new_f64(40, self.paperspace_elevation));
        }

        // $THICKNESS
        pairs.push(CodePair::new_str(9, "$THICKNESS"));
        pairs.push(CodePair::new_f64(40, self.thickness));

        // $LIMCHECK
        pairs.push(CodePair::new_str(9, "$LIMCHECK"));
        pairs.push(CodePair::new_i16(70, as_i16(self.use_limits_checking)));

        // $BLIPMODE
        if self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$BLIPMODE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.blip_mode)));
        }

        // $CHAMFERA
        pairs.push(CodePair::new_str(9, "$CHAMFERA"));
        pairs.push(CodePair::new_f64(40, self.first_chamfer_distance));

        // $CHAMFERB
        pairs.push(CodePair::new_str(9, "$CHAMFERB"));
        pairs.push(CodePair::new_f64(40, self.second_chamfer_distance));

        // $CHAMFERC
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$CHAMFERC"));
            pairs.push(CodePair::new_f64(40, self.chamfer_length));
        }

        // $CHAMFERD
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$CHAMFERD"));
            pairs.push(CodePair::new_f64(40, self.chamfer_angle));
        }

        // $SKPOLY
        pairs.push(CodePair::new_str(9, "$SKPOLY"));
        pairs.push(CodePair::new_i16(70, self.polyline_sketch_mode as i16));

        // $TDCREATE
        pairs.push(CodePair::new_str(9, "$TDCREATE"));
        pairs.push(CodePair::new_f64(40, as_double_local(self.creation_date)));

        // $TDUCREATE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$TDUCREATE"));
            pairs.push(CodePair::new_f64(40, as_double_utc(self.creation_date_universal)));
        }

        // $TDUPDATE
        pairs.push(CodePair::new_str(9, "$TDUPDATE"));
        pairs.push(CodePair::new_f64(40, as_double_local(self.update_date)));

        // $TDUUPDATE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$TDUUPDATE"));
            pairs.push(CodePair::new_f64(40, as_double_utc(self.update_date_universal)));
        }

        // $TDINDWG
        pairs.push(CodePair::new_str(9, "$TDINDWG"));
        pairs.push(CodePair::new_f64(40, duration_as_double(self.time_in_drawing)));

        // $TDUSRTIMER
        pairs.push(CodePair::new_str(9, "$TDUSRTIMER"));
        pairs.push(CodePair::new_f64(40, duration_as_double(self.user_elapsed_timer)));

        // $USRTIMER
        pairs.push(CodePair::new_str(9, "$USRTIMER"));
        pairs.push(CodePair::new_i16(70, as_i16(self.user_timer_on)));

        // $ANGBASE
        pairs.push(CodePair::new_str(9, "$ANGBASE"));
        pairs.push(CodePair::new_f64(50, self.angle_zero_direction));

        // $ANGDIR
        pairs.push(CodePair::new_str(9, "$ANGDIR"));
        pairs.push(CodePair::new_i16(70, self.angle_direction as i16));

        // $PDMODE
        pairs.push(CodePair::new_str(9, "$PDMODE"));
        pairs.push(CodePair::new_i16(70, self.point_display_mode as i16));

        // $PDSIZE
        pairs.push(CodePair::new_str(9, "$PDSIZE"));
        pairs.push(CodePair::new_f64(40, self.point_display_size));

        // $PLINEWID
        pairs.push(CodePair::new_str(9, "$PLINEWID"));
        pairs.push(CodePair::new_f64(40, self.default_polyline_width));

        // $COORDS
        if self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$COORDS"));
            pairs.push(CodePair::new_i16(70, self.coordinate_display as i16));
        }

        // $SPLFRAME
        if self.version <= AcadVersion::R2013 {
            pairs.push(CodePair::new_str(9, "$SPLFRAME"));
            pairs.push(CodePair::new_i16(70, as_i16(self.display_spline_polygon_control)));
        }

        // $SPLINETYPE
        pairs.push(CodePair::new_str(9, "$SPLINETYPE"));
        pairs.push(CodePair::new_i16(70, self.pedit_spline_curve_type as i16));

        // $SPLINESEGS
        pairs.push(CodePair::new_str(9, "$SPLINESEGS"));
        pairs.push(CodePair::new_i16(70, self.line_segments_per_spline_patch));

        // $ATTDIA
        if self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$ATTDIA"));
            pairs.push(CodePair::new_i16(70, as_i16(self.show_attribute_entry_dialogs)));
        }

        // $ATTREQ
        if self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$ATTREQ"));
            pairs.push(CodePair::new_i16(70, as_i16(self.prompt_for_attribute_on_insert)));
        }

        // $HANDLING
        if self.version <= AcadVersion::R12 {
            pairs.push(CodePair::new_str(9, "$HANDLING"));
            pairs.push(CodePair::new_i16(70, as_i16(self.handles_enabled)));
        }

        // $HANDSEED
        pairs.push(CodePair::new_str(9, "$HANDSEED"));
        pairs.push(CodePair::new_string(5, &self.next_available_handle.as_string()));

        // $SURFTAB1
        pairs.push(CodePair::new_str(9, "$SURFTAB1"));
        pairs.push(CodePair::new_i16(70, self.mesh_tabulations_in_first_direction));

        // $SURFTAB2
        pairs.push(CodePair::new_str(9, "$SURFTAB2"));
        pairs.push(CodePair::new_i16(70, self.mesh_tabulations_in_second_direction));

        // $SURFTYPE
        pairs.push(CodePair::new_str(9, "$SURFTYPE"));
        pairs.push(CodePair::new_i16(70, self.pedit_smooth_surface_type as i16));

        // $SURFU
        pairs.push(CodePair::new_str(9, "$SURFU"));
        pairs.push(CodePair::new_i16(70, self.pedit_smooth_m_densith));

        // $SURFV
        pairs.push(CodePair::new_str(9, "$SURFV"));
        pairs.push(CodePair::new_i16(70, self.pedit_smooth_n_densith));

        // $UCSBASE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSBASE"));
            pairs.push(CodePair::new_string(2, &self.ucs_definition_name));
        }

        // $UCSNAME
        pairs.push(CodePair::new_str(9, "$UCSNAME"));
        pairs.push(CodePair::new_string(2, &self.ucs_name));

        // $UCSORG
        pairs.push(CodePair::new_str(9, "$UCSORG"));
        pairs.push(CodePair::new_f64(10, self.ucs_origin.x));
        pairs.push(CodePair::new_f64(20, self.ucs_origin.y));
        pairs.push(CodePair::new_f64(30, self.ucs_origin.z));

        // $UCSXDIR
        pairs.push(CodePair::new_str(9, "$UCSXDIR"));
        pairs.push(CodePair::new_f64(10, self.ucs_x_axis.x));
        pairs.push(CodePair::new_f64(20, self.ucs_x_axis.y));
        pairs.push(CodePair::new_f64(30, self.ucs_x_axis.z));

        // $UCSYDIR
        pairs.push(CodePair::new_str(9, "$UCSYDIR"));
        pairs.push(CodePair::new_f64(10, self.ucs_y_axis.x));
        pairs.push(CodePair::new_f64(20, self.ucs_y_axis.y));
        pairs.push(CodePair::new_f64(30, self.ucs_y_axis.z));

        // $UCSORTHOREF
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORTHOREF"));
            pairs.push(CodePair::new_string(2, &self.ortho_ucs_reference));
        }

        // $UCSORTHOVIEW
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORTHOVIEW"));
            pairs.push(CodePair::new_i16(70, self.orthgraphic_view_type as i16));
        }

        // $UCSORGTOP
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORGTOP"));
            pairs.push(CodePair::new_f64(10, self.ucs_origin_top.x));
            pairs.push(CodePair::new_f64(20, self.ucs_origin_top.y));
            pairs.push(CodePair::new_f64(30, self.ucs_origin_top.z));
        }

        // $UCSORGBOTTOM
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORGBOTTOM"));
            pairs.push(CodePair::new_f64(10, self.ucs_origin_bottom.x));
            pairs.push(CodePair::new_f64(20, self.ucs_origin_bottom.y));
            pairs.push(CodePair::new_f64(30, self.ucs_origin_bottom.z));
        }

        // $UCSORGLEFT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORGLEFT"));
            pairs.push(CodePair::new_f64(10, self.ucs_origin_left.x));
            pairs.push(CodePair::new_f64(20, self.ucs_origin_left.y));
            pairs.push(CodePair::new_f64(30, self.ucs_origin_left.z));
        }

        // $UCSORGRIGHT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORGRIGHT"));
            pairs.push(CodePair::new_f64(10, self.ucs_origin_right.x));
            pairs.push(CodePair::new_f64(20, self.ucs_origin_right.y));
            pairs.push(CodePair::new_f64(30, self.ucs_origin_right.z));
        }

        // $UCSORGFRONT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORGFRONT"));
            pairs.push(CodePair::new_f64(10, self.ucs_origin_front.x));
            pairs.push(CodePair::new_f64(20, self.ucs_origin_front.y));
            pairs.push(CodePair::new_f64(30, self.ucs_origin_front.z));
        }

        // $UCSORGBACK
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$UCSORGBACK"));
            pairs.push(CodePair::new_f64(10, self.ucs_origin_back.x));
            pairs.push(CodePair::new_f64(20, self.ucs_origin_back.y));
            pairs.push(CodePair::new_f64(30, self.ucs_origin_back.z));
        }

        // $PUCSBASE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSBASE"));
            pairs.push(CodePair::new_string(2, &self.paperspace_ucs_definition_name));
        }

        // $PUCSNAME
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PUCSNAME"));
            pairs.push(CodePair::new_string(2, &self.paperspace_ucs_name));
        }

        // $PUCSORG
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PUCSORG"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin.z));
        }

        // $PUCSXDIR
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PUCSXDIR"));
            pairs.push(CodePair::new_f64(10, self.paperspace_x_axis.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_x_axis.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_x_axis.z));
        }

        // $PUCSYDIR
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PUCSYDIR"));
            pairs.push(CodePair::new_f64(10, self.paperspace_y_axis.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_y_axis.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_y_axis.z));
        }

        // $PUCSORTHOREF
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORTHOREF"));
            pairs.push(CodePair::new_string(2, &self.paperspace_ortho_ucs_reference));
        }

        // $PUCSORTHOVIEW
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORTHOVIEW"));
            pairs.push(CodePair::new_i16(70, self.paperspace_orthographic_view_type as i16));
        }

        // $PUCSORGTOP
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORGTOP"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin_top.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin_top.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin_top.z));
        }

        // $PUCSORGBOTTOM
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORGBOTTOM"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin_bottom.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin_bottom.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin_bottom.z));
        }

        // $PUCSORGLEFT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORGLEFT"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin_left.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin_left.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin_left.z));
        }

        // $PUCSORGRIGHT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORGRIGHT"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin_right.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin_right.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin_right.z));
        }

        // $PUCSORGFRONT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORGFRONT"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin_front.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin_front.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin_front.z));
        }

        // $PUCSORGBACK
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PUCSORGBACK"));
            pairs.push(CodePair::new_f64(10, self.paperspace_ucs_origin_back.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_ucs_origin_back.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_ucs_origin_back.z));
        }

        // $USERI1
        pairs.push(CodePair::new_str(9, "$USERI1"));
        pairs.push(CodePair::new_i16(70, self.user_int1));

        // $USERI2
        pairs.push(CodePair::new_str(9, "$USERI2"));
        pairs.push(CodePair::new_i16(70, self.user_int2));

        // $USERI3
        pairs.push(CodePair::new_str(9, "$USERI3"));
        pairs.push(CodePair::new_i16(70, self.user_int3));

        // $USERI4
        pairs.push(CodePair::new_str(9, "$USERI4"));
        pairs.push(CodePair::new_i16(70, self.user_int4));

        // $USERI5
        pairs.push(CodePair::new_str(9, "$USERI5"));
        pairs.push(CodePair::new_i16(70, self.user_int5));

        // $USERR1
        pairs.push(CodePair::new_str(9, "$USERR1"));
        pairs.push(CodePair::new_f64(40, self.user_real1));

        // $USERR2
        pairs.push(CodePair::new_str(9, "$USERR2"));
        pairs.push(CodePair::new_f64(40, self.user_real2));

        // $USERR3
        pairs.push(CodePair::new_str(9, "$USERR3"));
        pairs.push(CodePair::new_f64(40, self.user_real3));

        // $USERR4
        pairs.push(CodePair::new_str(9, "$USERR4"));
        pairs.push(CodePair::new_f64(40, self.user_real4));

        // $USERR5
        pairs.push(CodePair::new_str(9, "$USERR5"));
        pairs.push(CodePair::new_f64(40, self.user_real5));

        // $WORLDVIEW
        pairs.push(CodePair::new_str(9, "$WORLDVIEW"));
        pairs.push(CodePair::new_i16(70, as_i16(self.set_ucs_to_wcs_in_d_view_or_v_point)));

        // $SHADEDGE
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$SHADEDGE"));
            pairs.push(CodePair::new_i16(70, self.edge_shading as i16));
        }

        // $SHADEDIF
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$SHADEDIF"));
            pairs.push(CodePair::new_i16(70, self.percent_ambient_to_diffuse));
        }

        // $TILEMODE
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$TILEMODE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.previous_release_tile_compatability)));
        }

        // $MAXACTVP
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$MAXACTVP"));
            pairs.push(CodePair::new_i16(70, self.maximum_active_viewports));
        }

        // $PINSBASE
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$PINSBASE"));
            pairs.push(CodePair::new_f64(10, self.paperspace_insertion_base.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_insertion_base.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_insertion_base.z));
        }

        // $PLIMCHECK
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PLIMCHECK"));
            pairs.push(CodePair::new_i16(70, as_i16(self.limit_checking_in_paperspace)));
        }

        // $PEXTMIN
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PEXTMIN"));
            pairs.push(CodePair::new_f64(10, self.paperspace_minimum_drawing_extents.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_minimum_drawing_extents.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_minimum_drawing_extents.z));
        }

        // $PEXTMAX
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PEXTMAX"));
            pairs.push(CodePair::new_f64(10, self.paperspace_maximum_drawing_extents.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_maximum_drawing_extents.y));
            pairs.push(CodePair::new_f64(30, self.paperspace_maximum_drawing_extents.z));
        }

        // $PLIMMIN
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PLIMMIN"));
            pairs.push(CodePair::new_f64(10, self.paperspace_minimum_drawing_limits.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_minimum_drawing_limits.y));
        }

        // $PLIMMAX
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PLIMMAX"));
            pairs.push(CodePair::new_f64(10, self.paperspace_maximum_drawing_limits.x));
            pairs.push(CodePair::new_f64(20, self.paperspace_maximum_drawing_limits.y));
        }

        // $UNITMODE
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$UNITMODE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.display_fractions_in_input)));
        }

        // $VISRETAIN
        if self.version >= AcadVersion::R12 {
            pairs.push(CodePair::new_str(9, "$VISRETAIN"));
            pairs.push(CodePair::new_i16(70, as_i16(self.retain_x_ref_dependent_visibility_settings)));
        }

        // $PLINEGEN
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PLINEGEN"));
            pairs.push(CodePair::new_i16(70, as_i16(self.is_polyline_continuous_around_verticies)));
        }

        // $PSLTSCALE
        if self.version >= AcadVersion::R11 {
            pairs.push(CodePair::new_str(9, "$PSLTSCALE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.scale_line_types_in_paperspace)));
        }

        // $TREEDEPTH
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$TREEDEPTH"));
            pairs.push(CodePair::new_i16(70, self.spacial_index_max_depth));
        }

        // $PICKSTYLE
        if self.version >= AcadVersion::R13 && self.version <= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$PICKSTYLE"));
            pairs.push(CodePair::new_i16(70, self.pick_style as i16));
        }

        // $CMLSTYLE
        if self.version == AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$CMLSTYLE"));
            pairs.push(CodePair::new_string(7, &self.current_multiline_style));
        }

        // $CMLSTYLE
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$CMLSTYLE"));
            pairs.push(CodePair::new_string(2, &self.current_multiline_style));
        }

        // $CMLJUST
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$CMLJUST"));
            pairs.push(CodePair::new_i16(70, self.current_multiline_justification as i16));
        }

        // $CMLSCALE
        if self.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(9, "$CMLSCALE"));
            pairs.push(CodePair::new_f64(40, self.current_multiline_scale));
        }

        // $PROXYGRAPHICS
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$PROXYGRAPHICS"));
            pairs.push(CodePair::new_i16(70, as_i16(self.save_proxy_graphics)));
        }

        // $MEASUREMENT
        if self.version >= AcadVersion::R14 {
            pairs.push(CodePair::new_str(9, "$MEASUREMENT"));
            pairs.push(CodePair::new_i16(70, self.drawing_units as i16));
        }

        // $CELWEIGHT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$CELWEIGHT"));
            pairs.push(CodePair::new_i16(370, self.new_object_line_weight.raw_value()));
        }

        // $ENDCAPS
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$ENDCAPS"));
            pairs.push(CodePair::new_i16(280, self.end_cap_setting as i16));
        }

        // $JOINSTYLE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$JOINSTYLE"));
            pairs.push(CodePair::new_i16(280, self.lineweight_joint_setting as i16));
        }

        // $LWDISPLAY
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$LWDISPLAY"));
            pairs.push(CodePair::new_bool(290, self.display_linewieght_in_model_and_layout_tab));
        }

        // $INSUNITS
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$INSUNITS"));
            pairs.push(CodePair::new_i16(70, self.default_drawing_units as i16));
        }

        // $HYPERLINKBASE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$HYPERLINKBASE"));
            pairs.push(CodePair::new_string(1, &self.hyperlink_base));
        }

        // $STYLESHEET
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$STYLESHEET"));
            pairs.push(CodePair::new_string(1, &self.stylesheet));
        }

        // $XEDIT
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$XEDIT"));
            pairs.push(CodePair::new_bool(290, self.can_use_in_place_reference_editing));
        }

        // $CEPSNID
        if self.version >= AcadVersion::R2000 && self.new_object_plot_style_handle != Handle::empty() {
            pairs.push(CodePair::new_str(9, "$CEPSNID"));
            pairs.push(CodePair::new_string(390, &self.new_object_plot_style_handle.as_string()));
        }

        // $CEPSNTYPE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$CEPSNTYPE"));
            pairs.push(CodePair::new_i16(380, self.new_object_plot_style as i16));
        }

        // $PSTYLEMODE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PSTYLEMODE"));
            pairs.push(CodePair::new_bool(290, self.uses_color_dependent_plot_style_tables));
        }

        // $FINGERPRINTGUID
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$FINGERPRINTGUID"));
            pairs.push(CodePair::new_string(2, &uuid_string(&self.fingerprint_guid)));
        }

        // $VERSIONGUID
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$VERSIONGUID"));
            pairs.push(CodePair::new_string(2, &uuid_string(&self.version_guid)));
        }

        // $EXTNAMES
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$EXTNAMES"));
            pairs.push(CodePair::new_bool(290, self.use_acad2000_symbol_table_naming));
        }

        // $PSVPSCALE
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$PSVPSCALE"));
            pairs.push(CodePair::new_f64(40, self.viewport_view_scale_factor));
        }

        // $OLESTARTUP
        if self.version >= AcadVersion::R2000 {
            pairs.push(CodePair::new_str(9, "$OLESTARTUP"));
            pairs.push(CodePair::new_bool(290, self.ole_startup));
        }

        // $SORTENTS
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$SORTENTS"));
            pairs.push(CodePair::new_i16(280, self.object_sorting_methods_flags as i16));
        }

        // $INDEXCTL
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$INDEXCTL"));
            pairs.push(CodePair::new_i16(280, self.layer_and_spatial_index_save_mode as i16));
        }

        // $HIDETEXT
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$HIDETEXT"));
            pairs.push(CodePair::new_i16(280, as_i16(self.hide_text_objects_when_producint_hidden_view)));
        }

        // $XCLIPFRAME
        if self.version >= AcadVersion::R2004 && self.version <= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$XCLIPFRAME"));
            pairs.push(CodePair::new_bool(290, bool_from_clipping(self.is_x_ref_clipping_boundary_visible)));
        }

        // $XCLIPFRAME
        if self.version >= AcadVersion::R2010 {
            pairs.push(CodePair::new_str(9, "$XCLIPFRAME"));
            pairs.push(CodePair::new_i16(280, self.is_x_ref_clipping_boundary_visible as i16));
        }

        // $HALOGAP
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$HALOGAP"));
            pairs.push(CodePair::new_i16(280, self.halo_gap_percent as i16));
        }

        // $OBSCOLOR
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$OBSCOLOR"));
            pairs.push(CodePair::new_i16(70, self.obscured_line_color.raw_value()));
        }

        // $OBSLTYPE
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$OBSLTYPE"));
            pairs.push(CodePair::new_i16(280, self.obscured_line_type_style as i16));
        }

        // $INTERSECTIONDISPLAY
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$INTERSECTIONDISPLAY"));
            pairs.push(CodePair::new_i16(280, as_i16(self.display_intersection_polylines)));
        }

        // $INTERSECTIONCOLOR
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$INTERSECTIONCOLOR"));
            pairs.push(CodePair::new_i16(70, self.intersection_polyline_color.raw_value()));
        }

        // $DIMASSOC
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$DIMASSOC"));
            pairs.push(CodePair::new_i16(280, self.dimension_object_associativity as i16));
        }

        // $PROJECTNAME
        if self.version >= AcadVersion::R2004 {
            pairs.push(CodePair::new_str(9, "$PROJECTNAME"));
            pairs.push(CodePair::new_string(1, &self.project_name));
        }

        // $CAMERADISPLAY
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$CAMERADISPLAY"));
            pairs.push(CodePair::new_bool(290, self.use_camera_display));
        }

        // $LENSLENGTH
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LENSLENGTH"));
            pairs.push(CodePair::new_f64(40, self.lens_length));
        }

        // $CAMERAHEIGHT
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$CAMERAHEIGHT"));
            pairs.push(CodePair::new_f64(40, self.camera_height));
        }

        // $STEPSPERSEC
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$STEPSPERSEC"));
            pairs.push(CodePair::new_f64(40, self.steps_per_second_in_walk_or_fly_mode));
        }

        // $STEPSIZE
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$STEPSIZE"));
            pairs.push(CodePair::new_f64(40, self.step_size_in_walk_or_fly_mode));
        }

        // $3DDWFPREC
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$3DDWFPREC"));
            pairs.push(CodePair::new_f64(40, f64::from(self.dwf_3d_precision as i32)));
        }

        // $PSOLWIDTH
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$PSOLWIDTH"));
            pairs.push(CodePair::new_f64(40, self.last_poly_solid_width));
        }

        // $PSOLHEIGHT
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$PSOLHEIGHT"));
            pairs.push(CodePair::new_f64(40, self.last_poly_solid_height));
        }

        // $LOFTANG1
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LOFTANG1"));
            pairs.push(CodePair::new_f64(40, self.loft_operation_first_draft_angle));
        }

        // $LOFTANG2
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LOFTANG2"));
            pairs.push(CodePair::new_f64(40, self.loft_operation_second_draft_angle));
        }

        // $LOFTMAG1
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LOFTMAG1"));
            pairs.push(CodePair::new_f64(40, self.loft_operation_first_magnitude));
        }

        // $LOFTMAG2
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LOFTMAG2"));
            pairs.push(CodePair::new_f64(40, self.loft_operation_second_magnitude));
        }

        // $LOFTPARAM
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LOFTPARAM"));
            pairs.push(CodePair::new_i16(70, self.loft_flags as i16));
        }

        // $LOFTNORMALS
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LOFTNORMALS"));
            pairs.push(CodePair::new_i16(280, self.lofted_object_normal_mode as i16));
        }

        // $LATITUDE
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LATITUDE"));
            pairs.push(CodePair::new_f64(40, self.latitude));
        }

        // $LONGITUDE
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LONGITUDE"));
            pairs.push(CodePair::new_f64(40, self.longitude));
        }

        // $NORTHDIRECTION
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$NORTHDIRECTION"));
            pairs.push(CodePair::new_f64(40, self.angle_between_y_axis_and_north));
        }

        // $TIMEZONE
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$TIMEZONE"));
            pairs.push(CodePair::new_i16(70, self.time_zone as i16));
        }

        // $LIGHTGLYPHDISPLAY
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$LIGHTGLYPHDISPLAY"));
            pairs.push(CodePair::new_i16(280, as_i16(self.use_light_glyph_display)));
        }

        // $TILEMODELIGHTSYNCH
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$TILEMODELIGHTSYNCH"));
            pairs.push(CodePair::new_i16(280, as_i16(self.use_tile_mode_light_sync)));
        }

        // $CMATERIAL
        if self.version >= AcadVersion::R2007 && self.current_material_handle != Handle::empty() {
            pairs.push(CodePair::new_str(9, "$CMATERIAL"));
            pairs.push(CodePair::new_string(347, &self.current_material_handle.as_string()));
        }

        // $SOLIDHIST
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$SOLIDHIST"));
            pairs.push(CodePair::new_i16(280, as_i16(self.new_solids_contain_history)));
        }

        // $SHOWHIST
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$SHOWHIST"));
            pairs.push(CodePair::new_i16(280, self.solid_history_mode as i16));
        }

        // $DWFFRAME
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DWFFRAME"));
            pairs.push(CodePair::new_i16(280, self.dwf_underlay_frame_mode as i16));
        }

        // $DGNFRAME
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$DGNFRAME"));
            pairs.push(CodePair::new_i16(280, self.dgn_underlay_frame_mode as i16));
        }

        // $REALWORLDSCALE
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$REALWORLDSCALE"));
            pairs.push(CodePair::new_bool(290, self.use_real_world_scale));
        }

        // $INTERFERECOLOR
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$INTERFERECOLOR"));
            pairs.push(CodePair::new_i16(62, self.interference_object_color.raw_value()));
        }

        // $INTERFEREOBJVS
        if self.version >= AcadVersion::R2007 && self.interference_object_visual_style_pointer != Handle::empty() {
            pairs.push(CodePair::new_str(9, "$INTERFEREOBJVS"));
            pairs.push(CodePair::new_string(345, &self.interference_object_visual_style_pointer.as_string()));
        }

        // $INTERFEREVPVS
        if self.version >= AcadVersion::R2007 && self.interference_view_port_visual_style_pointer != Handle::empty() {
            pairs.push(CodePair::new_str(9, "$INTERFEREVPVS"));
            pairs.push(CodePair::new_string(346, &self.interference_view_port_visual_style_pointer.as_string()));
        }

        // $CSHADOW
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$CSHADOW"));
            pairs.push(CodePair::new_i16(280, self.shadow_mode as i16));
        }

        // $SHADOWPLANELOCATION
        if self.version >= AcadVersion::R2007 {
            pairs.push(CodePair::new_str(9, "$SHADOWPLANELOCATION"));
            pairs.push(CodePair::new_f64(40, self.shadow_plane_z_offset));
        }

        // $AXISMODE
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$AXISMODE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.axis_on)));
        }

        // $AXISUNIT
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$AXISUNIT"));
            pairs.push(CodePair::new_f64(10, self.axis_tick_spacing.x));
            pairs.push(CodePair::new_f64(20, self.axis_tick_spacing.y));
        }

        // $FASTZOOM
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$FASTZOOM"));
            pairs.push(CodePair::new_i16(70, as_i16(self.fast_zoom)));
        }

        // $GRIDMODE
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$GRIDMODE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.grid_on)));
        }

        // $GRIDUNIT
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$GRIDUNIT"));
            pairs.push(CodePair::new_f64(10, self.grid_spacing.x));
            pairs.push(CodePair::new_f64(20, self.grid_spacing.y));
        }

        // $SNAPANG
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$SNAPANG"));
            pairs.push(CodePair::new_f64(50, self.snap_rotation_angle));
        }

        // $SNAPBASE
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$SNAPBASE"));
            pairs.push(CodePair::new_f64(10, self.snap_base_point.x));
            pairs.push(CodePair::new_f64(20, self.snap_base_point.y));
        }

        // $SNAPISOPAIR
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$SNAPISOPAIR"));
            pairs.push(CodePair::new_i16(70, self.snap_isometric_plane as i16));
        }

        // $SNAPMODE
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$SNAPMODE"));
            pairs.push(CodePair::new_i16(70, as_i16(self.snap_on)));
        }

        // $SNAPSTYLE
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$SNAPSTYLE"));
            pairs.push(CodePair::new_i16(70, self.snap_style as i16));
        }

        // $SNAPUNIT
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$SNAPUNIT"));
            pairs.push(CodePair::new_f64(10, self.snap_spacing.x));
            pairs.push(CodePair::new_f64(20, self.snap_spacing.y));
        }

        // $VIEWCTR
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$VIEWCTR"));
            pairs.push(CodePair::new_f64(10, self.view_center.x));
            pairs.push(CodePair::new_f64(20, self.view_center.y));
        }

        // $VIEWDIR
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$VIEWDIR"));
            pairs.push(CodePair::new_f64(10, self.view_direction.x));
            pairs.push(CodePair::new_f64(20, self.view_direction.y));
            pairs.push(CodePair::new_f64(30, self.view_direction.z));
        }

        // $VIEWSIZE
        if self.version <= AcadVersion::R10 {
            pairs.push(CodePair::new_str(9, "$VIEWSIZE"));
            pairs.push(CodePair::new_f64(40, self.view_height));
        }

    }
}
