// The contents of this file are automatically generated and should not be modified directly.  See the `build` directory.

use crate::{
    CodePair,
    Color,
    DataTableValue,
    Drawing,
    DrawingItem,
    DrawingItemMut,
    DxfError,
    DxfResult,
    ExtensionGroup,
    GeoMeshPoint,
    Handle,
    MLineStyleElement,
    Point,
    SectionTypeSettings,
    TableCellStyle,
    TransformationMatrix,
    Vector,
    XData,
};
use crate::code_pair_put_back::CodePairPutBack;
use crate::extension_data;
use crate::helper_functions::*;
use crate::tables::*;
use crate::x_data;

use crate::entities::*;
use crate::enums::*;
use enum_primitive::FromPrimitive;
use std::collections::HashMap;

use chrono::{DateTime, Local};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectCommon {
    pub handle: Handle,
    pub extension_data_groups: Vec<ExtensionGroup>,
    pub x_data: Vec<XData>,
    #[doc(hidden)]
    pub __owner_handle: Handle,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Object {
    pub common: ObjectCommon,
    pub specific: ObjectType,
}

impl Default for ObjectCommon {
    fn default() -> ObjectCommon {
        ObjectCommon {
            handle: Handle::empty(),
            extension_data_groups: vec![],
            x_data: vec![],
            __owner_handle: Handle::empty(),
        }
    }
}

impl ObjectCommon {
    pub fn owner<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__owner_handle)
    }
    pub fn set_owner(&mut self, item: &DrawingItemMut) {
        self.__owner_handle = item.handle();
    }
    pub(crate) fn apply_individual_pair(&mut self, pair: &CodePair, iter: &mut CodePairPutBack) -> DxfResult<bool> {
        match pair.code {
            5 => { self.handle = pair.as_handle()? },
            extension_data::EXTENSION_DATA_GROUP => {
                let group = ExtensionGroup::read_group(pair.assert_string()?, iter, pair.offset)?;
                self.extension_data_groups.push(group);
            },
            330 => { self.__owner_handle = pair.as_handle()? },
            x_data::XDATA_APPLICATIONNAME => {
                let x = XData::read_item(pair.assert_string()?, iter)?;
                self.x_data.push(x);
            },
            _ => return Ok(false), // unknown code
        }
        Ok(true)
    }
    pub(crate) fn add_code_pairs(&self, pairs: &mut Vec<CodePair>, version: AcadVersion) {
        let obj = self;
        pairs.push(CodePair::new_string(5, &self.handle.as_string()));
        if version >= AcadVersion::R14 {
            for group in &self.extension_data_groups {
                group.add_code_pairs(pairs);
            }
        }
        if obj.__owner_handle != Handle(0) {
            pairs.push(CodePair::new_string(330, &obj.__owner_handle.as_string()));
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectType {
    AcadProxyObject(AcadProxyObject),
    DictionaryWithDefault(DictionaryWithDefault),
    PlaceHolder(PlaceHolder),
    NavisWorksModelDefinition(NavisWorksModelDefinition),
    DataTable(DataTable),
    Dictionary(Dictionary),
    DictionaryVariable(DictionaryVariable),
    DimensionAssoc(DimensionAssoc),
    Field(Field),
    GeoData(GeoData),
    Group(Group),
    IdBuffer(IdBuffer),
    ImageDefinition(ImageDefinition),
    ImageDefinitionReactor(ImageDefinitionReactor),
    LayerFilter(LayerFilter),
    LayerIndex(LayerIndex),
    Layout(Layout),
    LightList(LightList),
    Material(Material),
    MLeaderStyle(MLeaderStyle),
    MLineStyle(MLineStyle),
    ObjectPointer(ObjectPointer),
    PlotSettings(PlotSettings),
    RapidRTRenderEnvironment(RapidRTRenderEnvironment),
    RapidRenderSettings(RapidRenderSettings),
    RasterVariables(RasterVariables),
    MentalRayRenderSettings(MentalRayRenderSettings),
    RenderEnvironment(RenderEnvironment),
    RenderGlobal(RenderGlobal),
    SectionManager(SectionManager),
    SectionSettings(SectionSettings),
    SortentsTable(SortentsTable),
    SpatialFilter(SpatialFilter),
    SpatialIndex(SpatialIndex),
    SunStudy(SunStudy),
    TableStyle(TableStyle),
    UnderlayDefinition(UnderlayDefinition),
    VbaProject(VbaProject),
    VisualStyle(VisualStyle),
    WipeoutVariables(WipeoutVariables),
    XRecordObject(XRecordObject),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct AcadProxyObject {
    pub proxy_object_class_id: i32,
    pub application_object_class_id: i32,
    pub size_in_bits: i32,
    pub binary_object_data: Vec<Vec<u8>>,
    pub object_ids: Vec<String>,
    #[doc(hidden)]
    pub __object_ids_a: Vec<String>,
    #[doc(hidden)]
    pub __object_ids_b: Vec<String>,
    #[doc(hidden)]
    pub __object_ids_c: Vec<String>,
    #[doc(hidden)]
    pub __object_ids_d: Vec<String>,
    #[doc(hidden)]
    pub __object_drawing_format: u32,
    pub is_original_object_format: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for AcadProxyObject {
    fn default() -> AcadProxyObject {
        AcadProxyObject {
            proxy_object_class_id: 499,
            application_object_class_id: 500,
            size_in_bits: 0,
            binary_object_data: vec![],
            object_ids: vec![],
            __object_ids_a: vec![],
            __object_ids_b: vec![],
            __object_ids_c: vec![],
            __object_ids_d: vec![],
            __object_drawing_format: 0,
            is_original_object_format: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DictionaryWithDefault {
    pub duplicate_record_handling: DictionaryDuplicateRecordHandling,
    pub default_handle: Handle,
    pub value_handles: HashMap<String, Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for DictionaryWithDefault {
    fn default() -> DictionaryWithDefault {
        DictionaryWithDefault {
            duplicate_record_handling: DictionaryDuplicateRecordHandling::NotApplicable,
            default_handle: Handle::empty(),
            value_handles: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceHolder {
}

#[allow(clippy::derivable_impls)]
impl Default for PlaceHolder {
    fn default() -> PlaceHolder {
        PlaceHolder {
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct NavisWorksModelDefinition {
    pub model_path: String,
    pub is_model_loaded_on_drawing_open: bool,
    pub minimum_extent: Point,
    pub maximum_extent: Point,
    pub is_host_geometry_drawn: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for NavisWorksModelDefinition {
    fn default() -> NavisWorksModelDefinition {
        NavisWorksModelDefinition {
            model_path: String::new(),
            is_model_loaded_on_drawing_open: false,
            minimum_extent: Point::origin(),
            maximum_extent: Point::origin(),
            is_host_geometry_drawn: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DataTable {
    pub field: i16,
    pub column_count: usize,
    pub row_count: usize,
    pub name: String,
    pub column_names: Vec<String>,
    pub values: Vec<Vec<Option<DataTableValue>>>,
}

#[allow(clippy::derivable_impls)]
impl Default for DataTable {
    fn default() -> DataTable {
        DataTable {
            field: 0,
            column_count: 0,
            row_count: 0,
            name: String::new(),
            column_names: vec![],
            values: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Dictionary {
    pub is_hard_owner: bool,
    pub duplicate_record_handling: DictionaryDuplicateRecordHandling,
    pub value_handles: HashMap<String, Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for Dictionary {
    fn default() -> Dictionary {
        Dictionary {
            is_hard_owner: false,
            duplicate_record_handling: DictionaryDuplicateRecordHandling::NotApplicable,
            value_handles: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DictionaryVariable {
    pub object_schema_number: i16,
    pub value: String,
}

#[allow(clippy::derivable_impls)]
impl Default for DictionaryVariable {
    fn default() -> DictionaryVariable {
        DictionaryVariable {
            object_schema_number: 0,
            value: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct DimensionAssoc {
    #[doc(hidden)]
    pub __dimension_handle: Handle,
    pub associativity_flags: i32,
    pub is_trans_space: bool,
    pub rotated_dimension_type: RotatedDimensionType,
    pub class_name: String,
    pub object_osnap_type: ObjectOsnapType,
    #[doc(hidden)]
    pub __main_object_handle: Handle,
    pub main_object_subentity_type: SubentityType,
    pub main_object_gs_marker_index: i32,
    #[doc(hidden)]
    pub __main_object_xref_handle: Handle,
    pub near_osnap_geometry_parameter: f64,
    pub osnap_point: Point,
    #[doc(hidden)]
    pub __intersection_object_handle: Handle,
    pub intersection_subentity_type: SubentityType,
    pub intersection_object_gs_marker_index: i32,
    #[doc(hidden)]
    pub __insertion_object_xref_handle: Handle,
    pub has_last_point_reference: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for DimensionAssoc {
    fn default() -> DimensionAssoc {
        DimensionAssoc {
            __dimension_handle: Handle::empty(),
            associativity_flags: 0,
            is_trans_space: true,
            rotated_dimension_type: RotatedDimensionType::Parallel,
            class_name: String::from("AcDbOsnapPointRef"),
            object_osnap_type: ObjectOsnapType::None,
            __main_object_handle: Handle::empty(),
            main_object_subentity_type: SubentityType::Edge,
            main_object_gs_marker_index: 0,
            __main_object_xref_handle: Handle::empty(),
            near_osnap_geometry_parameter: 0.0,
            osnap_point: Point::origin(),
            __intersection_object_handle: Handle::empty(),
            intersection_subentity_type: SubentityType::Edge,
            intersection_object_gs_marker_index: 0,
            __insertion_object_xref_handle: Handle::empty(),
            has_last_point_reference: false,
        }
    }
}

impl DimensionAssoc {
    pub fn is_first_point_reference(&self) -> bool {
        self.associativity_flags & 1 != 0
    }
    pub fn set_is_first_point_reference(&mut self, val: bool) {
        if val {
            self.associativity_flags |= 1;
        }
        else {
            self.associativity_flags &= !1;
        }
    }
    pub fn is_second_point_reference(&self) -> bool {
        self.associativity_flags & 2 != 0
    }
    pub fn set_is_second_point_reference(&mut self, val: bool) {
        if val {
            self.associativity_flags |= 2;
        }
        else {
            self.associativity_flags &= !2;
        }
    }
    pub fn is_third_point_reference(&self) -> bool {
        self.associativity_flags & 4 != 0
    }
    pub fn set_is_third_point_reference(&mut self, val: bool) {
        if val {
            self.associativity_flags |= 4;
        }
        else {
            self.associativity_flags &= !4;
        }
    }
    pub fn is_fourth_point_reference(&self) -> bool {
        self.associativity_flags & 8 != 0
    }
    pub fn set_is_fourth_point_reference(&mut self, val: bool) {
        if val {
            self.associativity_flags |= 8;
        }
        else {
            self.associativity_flags &= !8;
        }
    }
    pub fn dimension<'a>(&self, drawing: &'a Drawing) -> Option<&'a Entity> {
        match drawing.item_by_handle(self.__dimension_handle) {
            Some(DrawingItem::Entity(val)) => Some(val),
            _ => None,
        }
    }
    pub fn set_dimension(&mut self, item: &Entity) {
        self.__dimension_handle = DrawingItem::Entity(item).handle();
    }
    pub fn main_object<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__main_object_handle)
    }
    pub fn set_main_object(&mut self, item: &DrawingItemMut) {
        self.__main_object_handle = item.handle();
    }
    pub fn main_object_xref<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__main_object_xref_handle)
    }
    pub fn set_main_object_xref(&mut self, item: &DrawingItemMut) {
        self.__main_object_xref_handle = item.handle();
    }
    pub fn intersection_object<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__intersection_object_handle)
    }
    pub fn set_intersection_object(&mut self, item: &DrawingItemMut) {
        self.__intersection_object_handle = item.handle();
    }
    pub fn insertion_object_xref<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__insertion_object_xref_handle)
    }
    pub fn set_insertion_object_xref(&mut self, item: &DrawingItemMut) {
        self.__insertion_object_xref_handle = item.handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    pub evaluator_id: String,
    pub field_code_string: String,
    pub field_code_string_overflow: String,
    #[doc(hidden)]
    pub __format_string: String,
    pub evaluation_error_message: String,
    #[doc(hidden)]
    pub __child_field_count: i32,
    #[doc(hidden)]
    pub __child_fields_handle: Vec<Handle>,
    pub evaluation_option: i32,
    pub filling_option: i32,
    pub field_state: i32,
    pub evaluation_status: i32,
    pub evaluation_error_code: i32,
    #[doc(hidden)]
    pub __object_id_count: i32,
    #[doc(hidden)]
    pub __objects_handle: Vec<Handle>,
    #[doc(hidden)]
    pub __data_set_count: i32,
    pub field_data_keys: Vec<String>,
    pub evaluated_cache_key: String,
    #[doc(hidden)]
    pub __value_type_code: i32,
    #[doc(hidden)]
    pub __long_value: i32,
    #[doc(hidden)]
    pub __double_value: f64,
    #[doc(hidden)]
    pub __id_value: Handle,
    #[doc(hidden)]
    pub __binary_data_buffer_size: i32,
    #[doc(hidden)]
    pub __binary_data: Vec<u8>,
    #[doc(hidden)]
    pub __format_string_code301: String,
    #[doc(hidden)]
    pub __format_string_overflow: String,
    #[doc(hidden)]
    pub __format_string_length: i32,
    #[doc(hidden)]
    pub __child_field_count_value_type_code: Vec<i32>,
}

#[allow(clippy::derivable_impls)]
impl Default for Field {
    fn default() -> Field {
        Field {
            evaluator_id: String::new(),
            field_code_string: String::new(),
            field_code_string_overflow: String::new(),
            __format_string: String::new(),
            evaluation_error_message: String::new(),
            __child_field_count: 0,
            __child_fields_handle: vec![],
            evaluation_option: 0,
            filling_option: 0,
            field_state: 0,
            evaluation_status: 0,
            evaluation_error_code: 0,
            __object_id_count: 0,
            __objects_handle: vec![],
            __data_set_count: 0,
            field_data_keys: vec![],
            evaluated_cache_key: String::from("ACFD_FIELD_VALUE"),
            __value_type_code: 0,
            __long_value: 0,
            __double_value: 0.0,
            __id_value: Handle(0),
            __binary_data_buffer_size: 0,
            __binary_data: vec![],
            __format_string_code301: String::new(),
            __format_string_overflow: String::new(),
            __format_string_length: 0,
            __child_field_count_value_type_code: vec![],
        }
    }
}

impl Field {
    pub fn child_fields<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Object> {
        self.__child_fields_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Object(val)) => {
                    match val.specific {
                        ObjectType::Field(_) => Some(val),
                        _ => None,
                    }
                },
                _ => None,
            }
        }).collect()
    }
    pub fn add_child_fields(&mut self, item: &Object) -> DxfResult<()> {
        match item.specific {
            ObjectType::Field { .. } => self.__child_fields_handle.push(item.common.handle),
            _ => return Err(DxfError::WrongItemType),
        }

        Ok(())
    }
    pub fn objects<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Object> {
        self.__objects_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Object(val)) => Some(val),
                _ => None,
            }
        }).collect()
    }
    pub fn add_objects(&mut self, item: &Object) {
        self.__objects_handle.push(DrawingItem::Object(item).handle());
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoData {
    pub version: GeoDataVersion,
    pub coordinate_type: DesignCoordinateType,
    pub design_point: Point,
    pub reference_point: Point,
    pub north_vector: Vector,
    pub horizontal_unit_scale: f64,
    pub vertical_unit_scale: f64,
    pub horizontal_units: Units,
    pub vertical_units: Units,
    pub up_direction: Vector,
    pub scale_estimation_method: ScaleEstimationMethod,
    pub use_sea_level_correction: bool,
    pub user_specified_scale_factor: f64,
    pub sea_level_elevation: f64,
    pub coordinate_projection_radius: f64,
    pub coordinate_system_definition: String,
    pub geo_rss_tag: String,
    pub observation_from_tag: String,
    pub observation_to_tag: String,
    pub observation_coverage_tag: String,
    pub geo_mesh_points: Vec<GeoMeshPoint>,
    #[doc(hidden)]
    pub __geo_mesh_point_count: i32,
    #[doc(hidden)]
    pub __source_mesh_x_points: Vec<f64>,
    #[doc(hidden)]
    pub __source_mesh_y_points: Vec<f64>,
    #[doc(hidden)]
    pub __destination_mesh_x_points: Vec<f64>,
    #[doc(hidden)]
    pub __destination_mesh_y_points: Vec<f64>,
    pub face_indices: Vec<Point>,
    #[doc(hidden)]
    pub __faces_count: i32,
    #[doc(hidden)]
    pub __face_point_index_x: Vec<f64>,
    #[doc(hidden)]
    pub __face_point_index_y: Vec<f64>,
    #[doc(hidden)]
    pub __face_point_index_z: Vec<f64>,
}

#[allow(clippy::derivable_impls)]
impl Default for GeoData {
    fn default() -> GeoData {
        GeoData {
            version: GeoDataVersion::R2009,
            coordinate_type: DesignCoordinateType::Unknown,
            design_point: Point::origin(),
            reference_point: Point::origin(),
            north_vector: Vector::z_axis(),
            horizontal_unit_scale: 1.0,
            vertical_unit_scale: 1.0,
            horizontal_units: Units::Unitless,
            vertical_units: Units::Unitless,
            up_direction: Vector::z_axis(),
            scale_estimation_method: ScaleEstimationMethod::None,
            use_sea_level_correction: false,
            user_specified_scale_factor: 1.0,
            sea_level_elevation: 0.0,
            coordinate_projection_radius: 0.0,
            coordinate_system_definition: String::new(),
            geo_rss_tag: String::new(),
            observation_from_tag: String::new(),
            observation_to_tag: String::new(),
            observation_coverage_tag: String::new(),
            geo_mesh_points: vec![],
            __geo_mesh_point_count: 0,
            __source_mesh_x_points: vec![],
            __source_mesh_y_points: vec![],
            __destination_mesh_x_points: vec![],
            __destination_mesh_y_points: vec![],
            face_indices: vec![],
            __faces_count: 0,
            __face_point_index_x: vec![],
            __face_point_index_y: vec![],
            __face_point_index_z: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Group {
    pub description: String,
    pub is_named: bool,
    pub is_selectable: bool,
    #[doc(hidden)]
    pub __entities_handle: Vec<Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for Group {
    fn default() -> Group {
        Group {
            description: String::new(),
            is_named: true,
            is_selectable: true,
            __entities_handle: vec![],
        }
    }
}

impl Group {
    pub fn entities<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Entity> {
        self.__entities_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Entity(val)) => Some(val),
                _ => None,
            }
        }).collect()
    }
    pub fn add_entities(&mut self, item: &Entity) {
        self.__entities_handle.push(DrawingItem::Entity(item).handle());
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct IdBuffer {
    #[doc(hidden)]
    pub __entities_handle: Vec<Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for IdBuffer {
    fn default() -> IdBuffer {
        IdBuffer {
            __entities_handle: vec![],
        }
    }
}

impl IdBuffer {
    pub fn entities<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Entity> {
        self.__entities_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Entity(val)) => Some(val),
                _ => None,
            }
        }).collect()
    }
    pub fn add_entities(&mut self, item: &Entity) {
        self.__entities_handle.push(DrawingItem::Entity(item).handle());
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageDefinition {
    pub class_version: i32,
    pub file_path: String,
    pub image_width: i32,
    pub image_height: i32,
    pub pixel_width: f64,
    pub pixel_height: f64,
    pub is_image_loaded: bool,
    pub resolution_units: ImageResolutionUnits,
}

#[allow(clippy::derivable_impls)]
impl Default for ImageDefinition {
    fn default() -> ImageDefinition {
        ImageDefinition {
            class_version: 0,
            file_path: String::new(),
            image_width: 0,
            image_height: 0,
            pixel_width: 0.0,
            pixel_height: 0.0,
            is_image_loaded: true,
            resolution_units: ImageResolutionUnits::NoUnits,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageDefinitionReactor {
    pub class_version: i32,
}

#[allow(clippy::derivable_impls)]
impl Default for ImageDefinitionReactor {
    fn default() -> ImageDefinitionReactor {
        ImageDefinitionReactor {
            class_version: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LayerFilter {
    pub layer_names: Vec<String>,
}

#[allow(clippy::derivable_impls)]
impl Default for LayerFilter {
    fn default() -> LayerFilter {
        LayerFilter {
            layer_names: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LayerIndex {
    pub time_stamp: DateTime<Local>,
    pub layer_names: Vec<String>,
    #[doc(hidden)]
    pub __id_buffers_handle: Vec<Handle>,
    pub id_buffer_counts: Vec<i32>,
}

#[allow(clippy::derivable_impls)]
impl Default for LayerIndex {
    fn default() -> LayerIndex {
        LayerIndex {
            time_stamp: Local::now(),
            layer_names: vec![],
            __id_buffers_handle: vec![],
            id_buffer_counts: vec![],
        }
    }
}

impl LayerIndex {
    pub fn id_buffers<'a>(&self, drawing: &'a Drawing) -> Vec<DrawingItem<'a>> {
        self.__id_buffers_handle.iter().filter_map(|&h| drawing.item_by_handle(h)).collect()
    }
    pub fn add_id_buffers(&mut self, item: &DrawingItemMut) {
        self.__id_buffers_handle.push(item.handle());
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Layout {
    pub layout_name: String,
    pub layout_flags: i32,
    pub tab_order: i32,
    pub minimum_limits: Point,
    pub maximum_limits: Point,
    pub insertion_base_point: Point,
    pub minimum_extents: Point,
    pub maximum_extents: Point,
    pub elevation: f64,
    pub ucs_origin: Point,
    pub ucs_x_axis: Vector,
    pub ucs_y_axis: Vector,
    pub ucs_orthographic_type: UcsOrthographicType,
    #[doc(hidden)]
    pub __viewport_handle: Handle,
    #[doc(hidden)]
    pub __table_record_handle: Handle,
    #[doc(hidden)]
    pub __table_record_base_handle: Handle,
}

#[allow(clippy::derivable_impls)]
impl Default for Layout {
    fn default() -> Layout {
        Layout {
            layout_name: String::new(),
            layout_flags: 0,
            tab_order: 0,
            minimum_limits: Point::origin(),
            maximum_limits: Point::origin(),
            insertion_base_point: Point::origin(),
            minimum_extents: Point::origin(),
            maximum_extents: Point::origin(),
            elevation: 0.0,
            ucs_origin: Point::origin(),
            ucs_x_axis: Vector::x_axis(),
            ucs_y_axis: Vector::y_axis(),
            ucs_orthographic_type: UcsOrthographicType::NotOrthographic,
            __viewport_handle: Handle::empty(),
            __table_record_handle: Handle::empty(),
            __table_record_base_handle: Handle::empty(),
        }
    }
}

impl Layout {
    pub fn is_ps_lt_scale(&self) -> bool {
        self.layout_flags & 1 != 0
    }
    pub fn set_is_ps_lt_scale(&mut self, val: bool) {
        if val {
            self.layout_flags |= 1;
        }
        else {
            self.layout_flags &= !1;
        }
    }
    pub fn is_lim_check(&self) -> bool {
        self.layout_flags & 2 != 0
    }
    pub fn set_is_lim_check(&mut self, val: bool) {
        if val {
            self.layout_flags |= 2;
        }
        else {
            self.layout_flags &= !2;
        }
    }
    pub fn viewport<'a>(&self, drawing: &'a Drawing) -> Option<&'a ViewPort> {
        match drawing.item_by_handle(self.__viewport_handle) {
            Some(DrawingItem::ViewPort(val)) => Some(val),
            _ => None,
        }
    }
    pub fn set_viewport(&mut self, item: &ViewPort) {
        self.__viewport_handle = DrawingItem::ViewPort(item).handle();
    }
    pub fn table_record<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__table_record_handle)
    }
    pub fn set_table_record(&mut self, item: &DrawingItemMut) {
        self.__table_record_handle = item.handle();
    }
    pub fn table_record_base<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__table_record_base_handle)
    }
    pub fn set_table_record_base(&mut self, item: &DrawingItemMut) {
        self.__table_record_base_handle = item.handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LightList {
    pub version: i32,
    #[doc(hidden)]
    pub __lights_handle: Vec<Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for LightList {
    fn default() -> LightList {
        LightList {
            version: 0,
            __lights_handle: vec![],
        }
    }
}

impl LightList {
    pub fn lights<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Entity> {
        self.__lights_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Entity(val)) => {
                    match val.specific {
                        EntityType::Light(_) => Some(val),
                        _ => None,
                    }
                },
                _ => None,
            }
        }).collect()
    }
    pub fn add_lights(&mut self, item: &Entity) -> DxfResult<()> {
        match item.specific {
            EntityType::Light { .. } => self.__lights_handle.push(item.common.handle),
            _ => return Err(DxfError::WrongItemType),
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Material {
    pub name: String,
    pub description: String,
    pub override_ambient_color: bool,
    pub ambient_color_factor: f64,
    pub ambient_color_value: i32,
    pub override_diffuse_color: bool,
    pub diffuse_color_factor: f64,
    pub diffuse_color_value: i32,
    pub diffuse_map_blend_factor: f64,
    pub use_image_file_for_diffuse_map: bool,
    pub diffuse_map_file_name: String,
    pub diffuse_map_projection_method: MapProjectionMethod,
    pub diffuse_map_tiling_method: MapTilingMethod,
    pub diffuse_map_auto_transform_method: MapAutoTransformMethod,
    pub diffuse_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __diffuse_map_transformation_matrix_values: Vec<f64>,
    pub specular_gloss_factor: f64,
    pub override_specular_color: bool,
    pub specular_color_factor: f64,
    pub specular_color_value: i32,
    pub specular_map_blend_factor: f64,
    pub use_image_file_for_specular_map: bool,
    pub specular_map_file_name: String,
    pub specular_map_projection_method: MapProjectionMethod,
    pub specular_map_tiling_method: MapTilingMethod,
    pub specular_map_auto_transform_method: MapAutoTransformMethod,
    pub specular_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __specular_map_transformation_matrix_values: Vec<f64>,
    pub reflection_map_blend_factor: f64,
    pub use_image_file_for_reflection_map: bool,
    pub reflection_map_file_name: String,
    pub reflection_map_projection_method: MapProjectionMethod,
    pub reflection_map_tiling_method: MapTilingMethod,
    pub reflection_map_auto_transform_method: MapAutoTransformMethod,
    pub reflection_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __reflection_map_transformation_matrix_values: Vec<f64>,
    pub opacity_factor: f64,
    pub opacity_map_blend_factor: f64,
    pub use_image_file_for_opacity_map: bool,
    pub opacity_map_file_name: String,
    pub opacity_map_projection_method: MapProjectionMethod,
    pub opacity_map_tiling_method: MapTilingMethod,
    pub opacity_map_auto_transform_method: MapAutoTransformMethod,
    pub opacity_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __opacity_map_transformation_matrix_values: Vec<f64>,
    pub bump_map_blend_factor: f64,
    pub use_image_file_for_bump_map: bool,
    pub bump_map_file_name: String,
    pub bump_map_projection_method: MapProjectionMethod,
    pub bump_map_tiling_method: MapTilingMethod,
    pub bump_map_auto_transform_method: MapAutoTransformMethod,
    pub bump_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __bump_map_transformation_matrix_values: Vec<f64>,
    pub refraction_index: f64,
    pub refraction_map_blend_factor: f64,
    pub use_image_file_for_refraction_map: bool,
    pub refraction_map_file_name: String,
    pub refraction_map_projection_method: MapProjectionMethod,
    pub refraction_map_tiling_method: MapTilingMethod,
    pub refraction_map_auto_transform_method: MapAutoTransformMethod,
    pub refraction_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __refraction_map_transformation_matrix_values: Vec<f64>,
    pub color_bleed_scale: f64,
    pub indirect_dump_scale: f64,
    pub reflectance_scale: f64,
    pub transmittance_scale: f64,
    pub is_two_sided: bool,
    pub luminance: f64,
    pub luminance_mode: i16,
    pub normal_map_method: i16,
    pub normal_map_strength: f64,
    pub normal_map_blend_factor: f64,
    pub use_image_file_for_normal_map: bool,
    pub normal_map_file_name: String,
    pub normal_map_projection_method: MapProjectionMethod,
    pub normal_map_tiling_method: MapTilingMethod,
    pub normal_map_auto_transform_method: MapAutoTransformMethod,
    pub normal_map_transformation_matrix: TransformationMatrix,
    #[doc(hidden)]
    pub __normal_map_transformation_matrix_values: Vec<f64>,
    pub is_anonymous: bool,
    pub global_illumination_mode: i16,
    pub final_gather_mode: i16,
    pub gen_proc_name: String,
    pub gen_proc_boolean_value: bool,
    pub gen_proc_integer_value: i16,
    pub gen_proc_real_value: f64,
    pub gen_proc_text_value: String,
    pub gen_proc_table_end: bool,
    pub gen_proc_color_index_value: Color,
    pub gen_proc_color_rgb_value: i32,
    pub gen_proc_color_name: String,
    pub map_u_tile: i16,
    pub map_v_tile: i16,
    pub translucence: f64,
    pub self_illumination: i32,
    pub reflectivity: f64,
    pub illumination_model: i32,
    pub channel_flags: i32,
}

#[allow(clippy::derivable_impls)]
impl Default for Material {
    fn default() -> Material {
        Material {
            name: String::new(),
            description: String::new(),
            override_ambient_color: false,
            ambient_color_factor: 1.0,
            ambient_color_value: 0,
            override_diffuse_color: false,
            diffuse_color_factor: 1.0,
            diffuse_color_value: 0,
            diffuse_map_blend_factor: 1.0,
            use_image_file_for_diffuse_map: false,
            diffuse_map_file_name: String::new(),
            diffuse_map_projection_method: MapProjectionMethod::Planar,
            diffuse_map_tiling_method: MapTilingMethod::Tile,
            diffuse_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            diffuse_map_transformation_matrix: TransformationMatrix::identity(),
            __diffuse_map_transformation_matrix_values: vec![],
            specular_gloss_factor: 0.5,
            override_specular_color: false,
            specular_color_factor: 1.0,
            specular_color_value: 0,
            specular_map_blend_factor: 1.0,
            use_image_file_for_specular_map: false,
            specular_map_file_name: String::new(),
            specular_map_projection_method: MapProjectionMethod::Planar,
            specular_map_tiling_method: MapTilingMethod::Tile,
            specular_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            specular_map_transformation_matrix: TransformationMatrix::identity(),
            __specular_map_transformation_matrix_values: vec![],
            reflection_map_blend_factor: 1.0,
            use_image_file_for_reflection_map: false,
            reflection_map_file_name: String::new(),
            reflection_map_projection_method: MapProjectionMethod::Planar,
            reflection_map_tiling_method: MapTilingMethod::Tile,
            reflection_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            reflection_map_transformation_matrix: TransformationMatrix::identity(),
            __reflection_map_transformation_matrix_values: vec![],
            opacity_factor: 1.0,
            opacity_map_blend_factor: 1.0,
            use_image_file_for_opacity_map: false,
            opacity_map_file_name: String::new(),
            opacity_map_projection_method: MapProjectionMethod::Planar,
            opacity_map_tiling_method: MapTilingMethod::Tile,
            opacity_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            opacity_map_transformation_matrix: TransformationMatrix::identity(),
            __opacity_map_transformation_matrix_values: vec![],
            bump_map_blend_factor: 1.0,
            use_image_file_for_bump_map: false,
            bump_map_file_name: String::new(),
            bump_map_projection_method: MapProjectionMethod::Planar,
            bump_map_tiling_method: MapTilingMethod::Tile,
            bump_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            bump_map_transformation_matrix: TransformationMatrix::identity(),
            __bump_map_transformation_matrix_values: vec![],
            refraction_index: 1.0,
            refraction_map_blend_factor: 1.0,
            use_image_file_for_refraction_map: false,
            refraction_map_file_name: String::new(),
            refraction_map_projection_method: MapProjectionMethod::Planar,
            refraction_map_tiling_method: MapTilingMethod::Tile,
            refraction_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            refraction_map_transformation_matrix: TransformationMatrix::identity(),
            __refraction_map_transformation_matrix_values: vec![],
            color_bleed_scale: 0.0,
            indirect_dump_scale: 0.0,
            reflectance_scale: 0.0,
            transmittance_scale: 0.0,
            is_two_sided: false,
            luminance: 0.0,
            luminance_mode: 0,
            normal_map_method: 0,
            normal_map_strength: 1.0,
            normal_map_blend_factor: 1.0,
            use_image_file_for_normal_map: false,
            normal_map_file_name: String::new(),
            normal_map_projection_method: MapProjectionMethod::Planar,
            normal_map_tiling_method: MapTilingMethod::Tile,
            normal_map_auto_transform_method: MapAutoTransformMethod::NoAutoTransform,
            normal_map_transformation_matrix: TransformationMatrix::identity(),
            __normal_map_transformation_matrix_values: vec![],
            is_anonymous: false,
            global_illumination_mode: 0,
            final_gather_mode: 0,
            gen_proc_name: String::new(),
            gen_proc_boolean_value: false,
            gen_proc_integer_value: 0,
            gen_proc_real_value: 0.0,
            gen_proc_text_value: String::new(),
            gen_proc_table_end: false,
            gen_proc_color_index_value: Color::by_layer(),
            gen_proc_color_rgb_value: 0,
            gen_proc_color_name: String::new(),
            map_u_tile: 0,
            map_v_tile: 0,
            translucence: 0.0,
            self_illumination: 0,
            reflectivity: 0.0,
            illumination_model: 0,
            channel_flags: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct MLeaderStyle {
    pub content_type: i16,
    pub draw_m_leader_order_type: i16,
    pub draw_leader_order_type: i16,
    pub max_leader_segment_count: i32,
    pub first_segment_angle_constraint: f64,
    pub second_segment_angle_constraint: f64,
    pub leader_line_type: i16,
    pub leader_line_color: i32,
    #[doc(hidden)]
    pub __line_leader_type_handle: Handle,
    pub leader_line_weight: i32,
    pub enable_landing: bool,
    pub landing_gap: f64,
    pub enable_dogleg: bool,
    pub dogleg_length: f64,
    pub m_leader_style_description: String,
    #[doc(hidden)]
    pub __arrowhead_handle: Handle,
    pub arrowhead_size: f64,
    pub default_m_text_contents: String,
    #[doc(hidden)]
    pub __m_text_style_handle: Handle,
    pub text_left_attachment_type: i16,
    pub text_angle_type: i16,
    pub text_alignment_type: i16,
    pub text_right_attachment_type: i16,
    pub text_color: i32,
    pub text_height: f64,
    pub enable_frame_text: bool,
    pub always_align_text_left: bool,
    pub align_gap: f64,
    #[doc(hidden)]
    pub __block_content_handle: Handle,
    pub block_content_color: i32,
    pub block_content_x_scale: f64,
    pub block_content_y_scale: f64,
    pub block_content_z_scale: f64,
    pub enable_block_content_scale: bool,
    pub block_content_rotation: f64,
    pub enable_block_content_rotation: bool,
    pub block_content_connection_type: i16,
    pub scale: f64,
    pub overwrite_field_value: bool,
    pub is_annotative: bool,
    pub break_gap_size: f64,
    pub text_attachment_direction: TextAttachmentDirection,
    pub bottom_text_attachment_direction: BottomTextAttachmentDirection,
    pub top_text_attachment_direction: TopTextAttachmentDirection,
}

#[allow(clippy::derivable_impls)]
impl Default for MLeaderStyle {
    fn default() -> MLeaderStyle {
        MLeaderStyle {
            content_type: 0,
            draw_m_leader_order_type: 0,
            draw_leader_order_type: 0,
            max_leader_segment_count: 0,
            first_segment_angle_constraint: 0.0,
            second_segment_angle_constraint: 0.0,
            leader_line_type: 0,
            leader_line_color: 0,
            __line_leader_type_handle: Handle::empty(),
            leader_line_weight: 0,
            enable_landing: true,
            landing_gap: 0.0,
            enable_dogleg: true,
            dogleg_length: 0.0,
            m_leader_style_description: String::new(),
            __arrowhead_handle: Handle::empty(),
            arrowhead_size: 0.0,
            default_m_text_contents: String::new(),
            __m_text_style_handle: Handle::empty(),
            text_left_attachment_type: 0,
            text_angle_type: 0,
            text_alignment_type: 0,
            text_right_attachment_type: 0,
            text_color: 0,
            text_height: 0.0,
            enable_frame_text: true,
            always_align_text_left: true,
            align_gap: 0.0,
            __block_content_handle: Handle::empty(),
            block_content_color: 0,
            block_content_x_scale: 1.0,
            block_content_y_scale: 1.0,
            block_content_z_scale: 1.0,
            enable_block_content_scale: true,
            block_content_rotation: 0.0,
            enable_block_content_rotation: true,
            block_content_connection_type: 0,
            scale: 1.0,
            overwrite_field_value: false,
            is_annotative: true,
            break_gap_size: 0.0,
            text_attachment_direction: TextAttachmentDirection::Horizontal,
            bottom_text_attachment_direction: BottomTextAttachmentDirection::Center,
            top_text_attachment_direction: TopTextAttachmentDirection::Center,
        }
    }
}

impl MLeaderStyle {
    pub fn line_leader_type<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__line_leader_type_handle)
    }
    pub fn set_line_leader_type(&mut self, item: &DrawingItemMut) {
        self.__line_leader_type_handle = item.handle();
    }
    pub fn arrowhead<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__arrowhead_handle)
    }
    pub fn set_arrowhead(&mut self, item: &DrawingItemMut) {
        self.__arrowhead_handle = item.handle();
    }
    pub fn m_text_style<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__m_text_style_handle)
    }
    pub fn set_m_text_style(&mut self, item: &DrawingItemMut) {
        self.__m_text_style_handle = item.handle();
    }
    pub fn block_content<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__block_content_handle)
    }
    pub fn set_block_content(&mut self, item: &DrawingItemMut) {
        self.__block_content_handle = item.handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct MLineStyle {
    pub style_name: String,
    #[doc(hidden)]
    pub __flags: i32,
    pub description: String,
    pub fill_color: Color,
    pub start_angle: f64,
    pub end_angle: f64,
    pub elements: Vec<MLineStyleElement>,
    #[doc(hidden)]
    pub __element_count: i32,
    #[doc(hidden)]
    pub __element_offsets: Vec<f64>,
    #[doc(hidden)]
    pub __element_colors: Vec<Color>,
    #[doc(hidden)]
    pub __element_line_types: Vec<String>,
}

#[allow(clippy::derivable_impls)]
impl Default for MLineStyle {
    fn default() -> MLineStyle {
        MLineStyle {
            style_name: String::new(),
            __flags: 0,
            description: String::new(),
            fill_color: Color::by_layer(),
            start_angle: 90.0,
            end_angle: 90.0,
            elements: vec![],
            __element_count: 0,
            __element_offsets: vec![],
            __element_colors: vec![],
            __element_line_types: vec![],
        }
    }
}

impl MLineStyle {
    pub fn fill_on(&self) -> bool {
        self.__flags & 1 != 0
    }
    pub fn set_fill_on(&mut self, val: bool) {
        if val {
            self.__flags |= 1;
        }
        else {
            self.__flags &= !1;
        }
    }
    pub fn display_miters(&self) -> bool {
        self.__flags & 2 != 0
    }
    pub fn set_display_miters(&mut self, val: bool) {
        if val {
            self.__flags |= 2;
        }
        else {
            self.__flags &= !2;
        }
    }
    pub fn start_square_end_cap(&self) -> bool {
        self.__flags & 16 != 0
    }
    pub fn set_start_square_end_cap(&mut self, val: bool) {
        if val {
            self.__flags |= 16;
        }
        else {
            self.__flags &= !16;
        }
    }
    pub fn start_inner_arcs_cap(&self) -> bool {
        self.__flags & 32 != 0
    }
    pub fn set_start_inner_arcs_cap(&mut self, val: bool) {
        if val {
            self.__flags |= 32;
        }
        else {
            self.__flags &= !32;
        }
    }
    pub fn start_round_cap(&self) -> bool {
        self.__flags & 64 != 0
    }
    pub fn set_start_round_cap(&mut self, val: bool) {
        if val {
            self.__flags |= 64;
        }
        else {
            self.__flags &= !64;
        }
    }
    pub fn end_square_cap(&self) -> bool {
        self.__flags & 256 != 0
    }
    pub fn set_end_square_cap(&mut self, val: bool) {
        if val {
            self.__flags |= 256;
        }
        else {
            self.__flags &= !256;
        }
    }
    pub fn end_inner_arcs_cap(&self) -> bool {
        self.__flags & 512 != 0
    }
    pub fn set_end_inner_arcs_cap(&mut self, val: bool) {
        if val {
            self.__flags |= 512;
        }
        else {
            self.__flags &= !512;
        }
    }
    pub fn end_round_cap(&self) -> bool {
        self.__flags & 1024 != 0
    }
    pub fn set_end_round_cap(&mut self, val: bool) {
        if val {
            self.__flags |= 1024;
        }
        else {
            self.__flags &= !1024;
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectPointer {
}

#[allow(clippy::derivable_impls)]
impl Default for ObjectPointer {
    fn default() -> ObjectPointer {
        ObjectPointer {
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PlotSettings {
    pub page_setup_name: String,
    pub printer_name: String,
    pub paper_size: String,
    pub plot_view_name: String,
    pub unprintable_left_margin_size: f64,
    pub unprintable_bottom_margin_size: f64,
    pub unprintable_right_margin_size: f64,
    pub unprintable_top_margin_size: f64,
    pub paper_width: f64,
    pub paper_height: f64,
    pub plot_origin_x_value: f64,
    pub plot_origin_y_value: f64,
    pub plot_window_lower_left_x_value: f64,
    pub plot_window_lower_left_y_value: f64,
    pub plot_window_upper_right_x_value: f64,
    pub plot_window_upper_right_y_value: f64,
    pub custom_print_scale_numerator: f64,
    pub custom_print_scale_denominator: f64,
    pub plot_layout_flags: i32,
    pub plot_paper_units: PlotPaperUnits,
    pub plot_rotation: PlotRotation,
    pub plot_type: PlotType,
    pub current_style_sheet: String,
    pub standard_scale: StandardScale,
    pub shade_plot_mode: ShadePlotMode,
    pub shade_plot_resolution_level: ShadePlotResolutionLevel,
    pub shade_plot_custom_dpi: i32,
    pub standard_scale_value: f64,
    pub paper_image_origin_x: f64,
    pub paper_image_origin_y: f64,
    #[doc(hidden)]
    pub __shade_plot_object_handle: Handle,
}

#[allow(clippy::derivable_impls)]
impl Default for PlotSettings {
    fn default() -> PlotSettings {
        PlotSettings {
            page_setup_name: String::new(),
            printer_name: String::new(),
            paper_size: String::new(),
            plot_view_name: String::new(),
            unprintable_left_margin_size: 0.0,
            unprintable_bottom_margin_size: 0.0,
            unprintable_right_margin_size: 0.0,
            unprintable_top_margin_size: 0.0,
            paper_width: 0.0,
            paper_height: 0.0,
            plot_origin_x_value: 0.0,
            plot_origin_y_value: 0.0,
            plot_window_lower_left_x_value: 0.0,
            plot_window_lower_left_y_value: 0.0,
            plot_window_upper_right_x_value: 0.0,
            plot_window_upper_right_y_value: 0.0,
            custom_print_scale_numerator: 0.0,
            custom_print_scale_denominator: 0.0,
            plot_layout_flags: 0,
            plot_paper_units: PlotPaperUnits::Inches,
            plot_rotation: PlotRotation::NoRotation,
            plot_type: PlotType::DrawingExtents,
            current_style_sheet: String::new(),
            standard_scale: StandardScale::ScaledToFit,
            shade_plot_mode: ShadePlotMode::AsDisplayed,
            shade_plot_resolution_level: ShadePlotResolutionLevel::Normal,
            shade_plot_custom_dpi: 100,
            standard_scale_value: 1.0,
            paper_image_origin_x: 0.0,
            paper_image_origin_y: 0.0,
            __shade_plot_object_handle: Handle::empty(),
        }
    }
}

impl PlotSettings {
    pub fn plot_viewport_borders(&self) -> bool {
        self.plot_layout_flags & 1 != 0
    }
    pub fn set_plot_viewport_borders(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 1;
        }
        else {
            self.plot_layout_flags &= !1;
        }
    }
    pub fn show_plot_styles(&self) -> bool {
        self.plot_layout_flags & 2 != 0
    }
    pub fn set_show_plot_styles(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 2;
        }
        else {
            self.plot_layout_flags &= !2;
        }
    }
    pub fn plot_centered(&self) -> bool {
        self.plot_layout_flags & 4 != 0
    }
    pub fn set_plot_centered(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 4;
        }
        else {
            self.plot_layout_flags &= !4;
        }
    }
    pub fn plot_hidden(&self) -> bool {
        self.plot_layout_flags & 8 != 0
    }
    pub fn set_plot_hidden(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 8;
        }
        else {
            self.plot_layout_flags &= !8;
        }
    }
    pub fn use_standard_scale(&self) -> bool {
        self.plot_layout_flags & 16 != 0
    }
    pub fn set_use_standard_scale(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 16;
        }
        else {
            self.plot_layout_flags &= !16;
        }
    }
    pub fn plot_plot_styles(&self) -> bool {
        self.plot_layout_flags & 32 != 0
    }
    pub fn set_plot_plot_styles(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 32;
        }
        else {
            self.plot_layout_flags &= !32;
        }
    }
    pub fn scale_line_weights(&self) -> bool {
        self.plot_layout_flags & 64 != 0
    }
    pub fn set_scale_line_weights(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 64;
        }
        else {
            self.plot_layout_flags &= !64;
        }
    }
    pub fn print_line_weights(&self) -> bool {
        self.plot_layout_flags & 128 != 0
    }
    pub fn set_print_line_weights(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 128;
        }
        else {
            self.plot_layout_flags &= !128;
        }
    }
    pub fn draw_viewports_first(&self) -> bool {
        self.plot_layout_flags & 512 != 0
    }
    pub fn set_draw_viewports_first(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 512;
        }
        else {
            self.plot_layout_flags &= !512;
        }
    }
    pub fn model_type(&self) -> bool {
        self.plot_layout_flags & 1024 != 0
    }
    pub fn set_model_type(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 1024;
        }
        else {
            self.plot_layout_flags &= !1024;
        }
    }
    pub fn update_paper(&self) -> bool {
        self.plot_layout_flags & 2048 != 0
    }
    pub fn set_update_paper(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 2048;
        }
        else {
            self.plot_layout_flags &= !2048;
        }
    }
    pub fn zoom_to_paper_on_update(&self) -> bool {
        self.plot_layout_flags & 4096 != 0
    }
    pub fn set_zoom_to_paper_on_update(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 4096;
        }
        else {
            self.plot_layout_flags &= !4096;
        }
    }
    pub fn initializing(&self) -> bool {
        self.plot_layout_flags & 8192 != 0
    }
    pub fn set_initializing(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 8192;
        }
        else {
            self.plot_layout_flags &= !8192;
        }
    }
    pub fn prev_plot_init(&self) -> bool {
        self.plot_layout_flags & 16384 != 0
    }
    pub fn set_prev_plot_init(&mut self, val: bool) {
        if val {
            self.plot_layout_flags |= 16384;
        }
        else {
            self.plot_layout_flags &= !16384;
        }
    }
    pub fn shade_plot_object<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__shade_plot_object_handle)
    }
    pub fn set_shade_plot_object(&mut self, item: &DrawingItemMut) {
        self.__shade_plot_object_handle = item.handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RapidRTRenderEnvironment {
    pub subclass_version: i32,
    pub is_enabled: bool,
    pub image_based_lignting_map_file_name: String,
    /// Rotation angle in degrees.
    pub rotation_angle: f64,
    pub is_map_as_background: bool,
    #[doc(hidden)]
    pub __custom_background_handle: Handle,
}

#[allow(clippy::derivable_impls)]
impl Default for RapidRTRenderEnvironment {
    fn default() -> RapidRTRenderEnvironment {
        RapidRTRenderEnvironment {
            subclass_version: 0,
            is_enabled: true,
            image_based_lignting_map_file_name: String::new(),
            rotation_angle: 0.0,
            is_map_as_background: true,
            __custom_background_handle: Handle::empty(),
        }
    }
}

impl RapidRTRenderEnvironment {
    pub fn custom_background<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__custom_background_handle)
    }
    pub fn set_custom_background(&mut self, item: &DrawingItemMut) {
        self.__custom_background_handle = item.handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RapidRenderSettings {
    pub subclass_version: i32,
    pub render_preset_name: String,
    pub are_materials_enabled: bool,
    pub is_texture_sampling_enabled: bool,
    pub are_back_faces_enabled: bool,
    pub are_shadows_enabled: bool,
    pub preview_image_file_name: String,
    pub render_preset_description: String,
    pub display_index: i32,
    pub is_predefined_render_present: bool,
    pub render_duration: RenderDuration,
    pub render_level_count_imit: i32,
    pub render_time_limit: i32,
    pub render_accuracy: RenderAccuracy,
    pub sampling_filter: SamplingFilterType,
    pub filter_width: f64,
    pub filter_height: f64,
}

#[allow(clippy::derivable_impls)]
impl Default for RapidRenderSettings {
    fn default() -> RapidRenderSettings {
        RapidRenderSettings {
            subclass_version: 0,
            render_preset_name: String::new(),
            are_materials_enabled: true,
            is_texture_sampling_enabled: true,
            are_back_faces_enabled: true,
            are_shadows_enabled: true,
            preview_image_file_name: String::new(),
            render_preset_description: String::new(),
            display_index: 0,
            is_predefined_render_present: true,
            render_duration: RenderDuration::ByTime,
            render_level_count_imit: 0,
            render_time_limit: 0,
            render_accuracy: RenderAccuracy::Low,
            sampling_filter: SamplingFilterType::Box,
            filter_width: 1.0,
            filter_height: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RasterVariables {
    pub class_version: i32,
    pub is_display_frame_image: bool,
    pub is_high_display_quality: bool,
    pub image_units: RasterImageUnits,
}

#[allow(clippy::derivable_impls)]
impl Default for RasterVariables {
    fn default() -> RasterVariables {
        RasterVariables {
            class_version: 0,
            is_display_frame_image: false,
            is_high_display_quality: false,
            image_units: RasterImageUnits::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct MentalRayRenderSettings {
    pub class_version_1: i32,
    pub preset_name: String,
    pub render_materials: bool,
    pub texture_sampling_quality: i32,
    pub render_back_faces: bool,
    pub render_shadows: bool,
    pub preview_file_name: String,
    pub class_version_2: i32,
    pub minimum_sampling_rate: i32,
    pub maximum_sampling_rate: i32,
    pub sampling_filter_type: SamplingFilterType,
    pub filter_width: f64,
    pub filter_height: f64,
    pub sampling_contrast_color_red: f64,
    pub sampling_contrast_color_green: f64,
    pub sampling_contrast_color_blue: f64,
    pub sampling_contrast_color_alpha: f64,
    pub shadow_mode: RenderShadowMode,
    pub map_shadows: bool,
    pub ray_tracing: bool,
    pub ray_tracing_depth_reflections: i32,
    pub ray_tracing_depth_refractions: i32,
    pub ray_tracing_depth_maximum: i32,
    pub use_global_illumination: bool,
    pub sample_count: i32,
    pub use_global_illumination_radius: bool,
    pub global_illumination_radius: f64,
    pub photons_per_light: i32,
    pub global_illumination_depth_reflections: i32,
    pub global_illumination_depth_refractions: i32,
    pub global_illumination_depth_maximum: i32,
    pub use_final_gather: bool,
    pub final_gather_ray_count: i32,
    pub use_final_gather_minimum_radius: bool,
    pub use_final_gather_maximum_radius: bool,
    pub use_final_gather_pixels: bool,
    pub final_gather_sample_radius_minimum: f64,
    pub final_gather_sample_radius_maximum: f64,
    pub luminance_scale: f64,
    pub diagnostic_mode: RenderDiagnosticMode,
    pub diagnostic_grid_mode: RenderDiagnosticGridMode,
    pub grid_size: f64,
    pub diagnostic_photon_mode: DiagnosticPhotonMode,
    pub diagnostic_bsp_mode: DiagnosticBSPMode,
    pub export_mi_statistics: bool,
    pub mi_statistics_file_name: String,
    pub tile_size: i32,
    pub tile_order: TileOrder,
    pub memory_limit: i32,
}

#[allow(clippy::derivable_impls)]
impl Default for MentalRayRenderSettings {
    fn default() -> MentalRayRenderSettings {
        MentalRayRenderSettings {
            class_version_1: 1,
            preset_name: String::new(),
            render_materials: false,
            texture_sampling_quality: 0,
            render_back_faces: false,
            render_shadows: false,
            preview_file_name: String::new(),
            class_version_2: 1,
            minimum_sampling_rate: 0,
            maximum_sampling_rate: 0,
            sampling_filter_type: SamplingFilterType::Box,
            filter_width: 0.0,
            filter_height: 0.0,
            sampling_contrast_color_red: 0.0,
            sampling_contrast_color_green: 0.0,
            sampling_contrast_color_blue: 0.0,
            sampling_contrast_color_alpha: 0.0,
            shadow_mode: RenderShadowMode::Simple,
            map_shadows: false,
            ray_tracing: false,
            ray_tracing_depth_reflections: 0,
            ray_tracing_depth_refractions: 0,
            ray_tracing_depth_maximum: 0,
            use_global_illumination: false,
            sample_count: 0,
            use_global_illumination_radius: false,
            global_illumination_radius: 0.0,
            photons_per_light: 0,
            global_illumination_depth_reflections: 0,
            global_illumination_depth_refractions: 0,
            global_illumination_depth_maximum: 0,
            use_final_gather: false,
            final_gather_ray_count: 0,
            use_final_gather_minimum_radius: false,
            use_final_gather_maximum_radius: false,
            use_final_gather_pixels: false,
            final_gather_sample_radius_minimum: 0.0,
            final_gather_sample_radius_maximum: 0.0,
            luminance_scale: 1.0,
            diagnostic_mode: RenderDiagnosticMode::Off,
            diagnostic_grid_mode: RenderDiagnosticGridMode::Object,
            grid_size: 1.0,
            diagnostic_photon_mode: DiagnosticPhotonMode::Density,
            diagnostic_bsp_mode: DiagnosticBSPMode::Depth,
            export_mi_statistics: false,
            mi_statistics_file_name: String::new(),
            tile_size: 0,
            tile_order: TileOrder::Hilbert,
            memory_limit: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderEnvironment {
    pub class_version: i32,
    pub is_fog_enabled: bool,
    pub is_background_fog_enabled: bool,
    pub fog_color_red: i32,
    pub fog_color_green: i32,
    pub fog_color_blue: i32,
    pub near_fog_density_percent: f64,
    pub far_fog_density_percent: f64,
    pub near_clipping_plane_distance_percent: f64,
    pub far_clipping_plane_distance_percent: f64,
    pub use_environment_image: bool,
    pub environment_image_file_name: String,
}

#[allow(clippy::derivable_impls)]
impl Default for RenderEnvironment {
    fn default() -> RenderEnvironment {
        RenderEnvironment {
            class_version: 1,
            is_fog_enabled: false,
            is_background_fog_enabled: false,
            fog_color_red: 0,
            fog_color_green: 0,
            fog_color_blue: 0,
            near_fog_density_percent: 0.0,
            far_fog_density_percent: 0.0,
            near_clipping_plane_distance_percent: 0.0,
            far_clipping_plane_distance_percent: 1.0,
            use_environment_image: false,
            environment_image_file_name: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderGlobal {
    pub class_version: i32,
    pub render_procedure: RenderProcedure,
    pub render_destination: RenderDestination,
    pub save_to_file: bool,
    pub save_to_file_name: String,
    pub image_width: i32,
    pub image_height: i32,
    pub use_predefined_presets_first: bool,
    pub use_high_info_level: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for RenderGlobal {
    fn default() -> RenderGlobal {
        RenderGlobal {
            class_version: 2,
            render_procedure: RenderProcedure::View,
            render_destination: RenderDestination::RenderWindow,
            save_to_file: false,
            save_to_file_name: String::new(),
            image_width: 0,
            image_height: 0,
            use_predefined_presets_first: false,
            use_high_info_level: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SectionManager {
    pub requires_full_update: bool,
    #[doc(hidden)]
    pub __section_count: i32,
    #[doc(hidden)]
    pub __section_entities_handle: Vec<Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for SectionManager {
    fn default() -> SectionManager {
        SectionManager {
            requires_full_update: false,
            __section_count: 0,
            __section_entities_handle: vec![],
        }
    }
}

impl SectionManager {
    pub fn section_entities<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Entity> {
        self.__section_entities_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Entity(val)) => Some(val),
                _ => None,
            }
        }).collect()
    }
    pub fn add_section_entities(&mut self, item: &Entity) {
        self.__section_entities_handle.push(DrawingItem::Entity(item).handle());
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SectionSettings {
    pub section_type: i32,
    pub is_generation_option: bool,
    #[doc(hidden)]
    pub __source_objects_handle: Vec<Handle>,
    #[doc(hidden)]
    pub __destination_object_handle: Handle,
    pub destination_file_name: String,
    pub geometry_settings: Vec<SectionTypeSettings>,
}

#[allow(clippy::derivable_impls)]
impl Default for SectionSettings {
    fn default() -> SectionSettings {
        SectionSettings {
            section_type: 0,
            is_generation_option: false,
            __source_objects_handle: vec![],
            __destination_object_handle: Handle::empty(),
            destination_file_name: String::new(),
            geometry_settings: vec![],
        }
    }
}

impl SectionSettings {
    pub fn source_objects<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Object> {
        self.__source_objects_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Object(val)) => Some(val),
                _ => None,
            }
        }).collect()
    }
    pub fn add_source_objects(&mut self, item: &Object) {
        self.__source_objects_handle.push(DrawingItem::Object(item).handle());
    }
    pub fn destination_object<'a>(&self, drawing: &'a Drawing) -> Option<&'a Object> {
        match drawing.item_by_handle(self.__destination_object_handle) {
            Some(DrawingItem::Object(val)) => Some(val),
            _ => None,
        }
    }
    pub fn set_destination_object(&mut self, item: &Object) {
        self.__destination_object_handle = DrawingItem::Object(item).handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SortentsTable {
    #[doc(hidden)]
    pub __entities_handle: Vec<Handle>,
    #[doc(hidden)]
    pub __sort_items_handle: Vec<Handle>,
}

#[allow(clippy::derivable_impls)]
impl Default for SortentsTable {
    fn default() -> SortentsTable {
        SortentsTable {
            __entities_handle: vec![],
            __sort_items_handle: vec![],
        }
    }
}

impl SortentsTable {
    pub fn entities<'a>(&self, drawing: &'a Drawing) -> Vec<&'a Entity> {
        self.__entities_handle.iter().filter_map(|&h| {
            match drawing.item_by_handle(h) {
                Some(DrawingItem::Entity(val)) => Some(val),
                _ => None,
            }
        }).collect()
    }
    pub fn add_entities(&mut self, item: &Entity) {
        self.__entities_handle.push(DrawingItem::Entity(item).handle());
    }
    pub fn sort_items<'a>(&self, drawing: &'a Drawing) -> Vec<DrawingItem<'a>> {
        self.__sort_items_handle.iter().filter_map(|&h| drawing.item_by_handle(h)).collect()
    }
    pub fn add_sort_items(&mut self, item: &DrawingItemMut) {
        self.__sort_items_handle.push(item.handle());
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SpatialFilter {
    pub clip_boundary_definition_points: Vec<Point>,
    pub clip_boundary_normal: Vector,
    pub clip_boundary_origin: Point,
    pub is_clip_boundary_enabled: bool,
    pub is_front_clipping_plane: bool,
    pub front_clipping_plane_distance: f64,
    pub is_back_clipping_plane: bool,
    pub back_clipping_plane_distance: f64,
    pub inverse_transformation_matrix: TransformationMatrix,
    pub transformation_matrix: TransformationMatrix,
}

#[allow(clippy::derivable_impls)]
impl Default for SpatialFilter {
    fn default() -> SpatialFilter {
        SpatialFilter {
            clip_boundary_definition_points: vec![],
            clip_boundary_normal: Vector::z_axis(),
            clip_boundary_origin: Point::origin(),
            is_clip_boundary_enabled: false,
            is_front_clipping_plane: true,
            front_clipping_plane_distance: 0.0,
            is_back_clipping_plane: false,
            back_clipping_plane_distance: 0.0,
            inverse_transformation_matrix: TransformationMatrix::identity(),
            transformation_matrix: TransformationMatrix::identity(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SpatialIndex {
    pub timestamp: DateTime<Local>,
}

#[allow(clippy::derivable_impls)]
impl Default for SpatialIndex {
    fn default() -> SpatialIndex {
        SpatialIndex {
            timestamp: Local::now(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SunStudy {
    pub version: i32,
    pub sun_setup_name: String,
    pub description: String,
    pub output_type: i16,
    pub sheet_set_name: String,
    pub use_subset: bool,
    pub sheet_subset_name: String,
    pub select_dates_from_calendar: bool,
    pub dates: Vec<DateTime<Local>>,
    pub select_range_of_dates: bool,
    pub start_time_seconds_past_midnight: i32,
    pub end_time_seconds_past_midnight: i32,
    pub interval_in_seconds: i32,
    pub hours: Vec<i32>,
    #[doc(hidden)]
    pub __page_setup_wizard_handle: Handle,
    #[doc(hidden)]
    pub __view_handle: Handle,
    #[doc(hidden)]
    pub __visual_style_handle: Handle,
    pub shade_plot_type: i16,
    pub viewports_per_page: i32,
    pub viewport_distribution_row_count: i32,
    pub viewport_distribution_column_count: i32,
    pub spacing: f64,
    pub lock_viewports: bool,
    pub label_viewports: bool,
    #[doc(hidden)]
    pub __text_style_handle: Handle,
}

#[allow(clippy::derivable_impls)]
impl Default for SunStudy {
    fn default() -> SunStudy {
        SunStudy {
            version: 0,
            sun_setup_name: String::new(),
            description: String::new(),
            output_type: 0,
            sheet_set_name: String::new(),
            use_subset: false,
            sheet_subset_name: String::new(),
            select_dates_from_calendar: false,
            dates: vec![],
            select_range_of_dates: false,
            start_time_seconds_past_midnight: 0,
            end_time_seconds_past_midnight: 0,
            interval_in_seconds: 0,
            hours: vec![],
            __page_setup_wizard_handle: Handle::empty(),
            __view_handle: Handle::empty(),
            __visual_style_handle: Handle::empty(),
            shade_plot_type: 0,
            viewports_per_page: 0,
            viewport_distribution_row_count: 0,
            viewport_distribution_column_count: 0,
            spacing: 0.0,
            lock_viewports: false,
            label_viewports: false,
            __text_style_handle: Handle::empty(),
        }
    }
}

impl SunStudy {
    pub fn page_setup_wizard<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__page_setup_wizard_handle)
    }
    pub fn set_page_setup_wizard(&mut self, item: &DrawingItemMut) {
        self.__page_setup_wizard_handle = item.handle();
    }
    pub fn view<'a>(&self, drawing: &'a Drawing) -> Option<&'a View> {
        match drawing.item_by_handle(self.__view_handle) {
            Some(DrawingItem::View(val)) => Some(val),
            _ => None,
        }
    }
    pub fn set_view(&mut self, item: &View) {
        self.__view_handle = DrawingItem::View(item).handle();
    }
    pub fn visual_style<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__visual_style_handle)
    }
    pub fn set_visual_style(&mut self, item: &DrawingItemMut) {
        self.__visual_style_handle = item.handle();
    }
    pub fn text_style<'a>(&self, drawing: &'a Drawing) -> Option<DrawingItem<'a>> {
        drawing.item_by_handle(self.__text_style_handle)
    }
    pub fn set_text_style(&mut self, item: &DrawingItemMut) {
        self.__text_style_handle = item.handle();
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct TableStyle {
    pub version: Version,
    pub description: String,
    pub flow_direction: FlowDirection,
    pub flags: i32,
    pub horizontal_cell_margin: f64,
    pub vertical_cell_margin: f64,
    pub is_title_suppressed: bool,
    pub is_column_heading_suppressed: bool,
    pub cell_styles: Vec<TableCellStyle>,
}

#[allow(clippy::derivable_impls)]
impl Default for TableStyle {
    fn default() -> TableStyle {
        TableStyle {
            version: Version::R2010,
            description: String::new(),
            flow_direction: FlowDirection::Down,
            flags: 0,
            horizontal_cell_margin: 0.06,
            vertical_cell_margin: 0.06,
            is_title_suppressed: false,
            is_column_heading_suppressed: false,
            cell_styles: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct UnderlayDefinition {
    pub file_name: String,
    pub name: String,
}

#[allow(clippy::derivable_impls)]
impl Default for UnderlayDefinition {
    fn default() -> UnderlayDefinition {
        UnderlayDefinition {
            file_name: String::new(),
            name: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct VbaProject {
    pub data: Vec<u8>,
    #[doc(hidden)]
    pub __hex_data: Vec<Vec<u8>>,
}

#[allow(clippy::derivable_impls)]
impl Default for VbaProject {
    fn default() -> VbaProject {
        VbaProject {
            data: vec![],
            __hex_data: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct VisualStyle {
    pub description: String,
    pub type_code: i16,
    pub face_lighting_model: FaceLightingModel,
    pub face_lighting_quality: FaceLightingQuality,
    pub face_color_mode: FaceColorMode,
    pub face_modifier: FaceModifier,
    pub face_opacity_level: f64,
    pub face_specular_level: f64,
    pub color1: Color,
    pub color2: Color,
    pub face_style_mono_color: i32,
    pub edge_style_model: EdgeStyleModel,
    pub edge_style: i32,
    pub edge_intersection_color: Color,
    pub edge_obscured_color: Color,
    pub edge_obscured_line_type: i16,
    pub edge_intersection_line_type: i16,
    pub edge_crease_angle: f64,
    pub edge_modifiers: i32,
    pub edge_color: Color,
    pub edge_opacity_level: f64,
    pub edge_width: i16,
    pub edge_overhang: i16,
    pub edge_jitter: i16,
    pub edge_silhouette_color: Color,
    pub edge_silhouette_width: i16,
    pub edge_halo_gap: i16,
    pub edge_iso_line_count: i32,
    pub hide_edge_line_precision: bool,
    pub edge_style_apply_flags: i32,
    pub display_style_settings: i32,
    pub brightness: f64,
    pub shadow_type: i16,
    pub internal_flag: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for VisualStyle {
    fn default() -> VisualStyle {
        VisualStyle {
            description: String::new(),
            type_code: 0,
            face_lighting_model: FaceLightingModel::Visible,
            face_lighting_quality: FaceLightingQuality::PerFace,
            face_color_mode: FaceColorMode::ObjectColor,
            face_modifier: FaceModifier::None,
            face_opacity_level: 0.0,
            face_specular_level: 0.0,
            color1: Color::by_block(),
            color2: Color::by_block(),
            face_style_mono_color: 0,
            edge_style_model: EdgeStyleModel::NoEdges,
            edge_style: 0,
            edge_intersection_color: Color::by_block(),
            edge_obscured_color: Color::by_block(),
            edge_obscured_line_type: 0,
            edge_intersection_line_type: 0,
            edge_crease_angle: 0.0,
            edge_modifiers: 0,
            edge_color: Color::by_block(),
            edge_opacity_level: 0.0,
            edge_width: 0,
            edge_overhang: 0,
            edge_jitter: 0,
            edge_silhouette_color: Color::by_block(),
            edge_silhouette_width: 0,
            edge_halo_gap: 0,
            edge_iso_line_count: 0,
            hide_edge_line_precision: false,
            edge_style_apply_flags: 0,
            display_style_settings: 0,
            brightness: 0.0,
            shadow_type: 0,
            internal_flag: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct WipeoutVariables {
    pub class_version: i32,
    pub display_image_frame: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for WipeoutVariables {
    fn default() -> WipeoutVariables {
        WipeoutVariables {
            class_version: 0,
            display_image_frame: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct XRecordObject {
    pub duplicate_record_handling: DictionaryDuplicateRecordHandling,
    pub data_pairs: Vec<CodePair>,
}

#[allow(clippy::derivable_impls)]
impl Default for XRecordObject {
    fn default() -> XRecordObject {
        XRecordObject {
            duplicate_record_handling: DictionaryDuplicateRecordHandling::NotApplicable,
            data_pairs: vec![],
        }
    }
}

impl ObjectType {
    pub(crate) fn is_supported_on_version(&self, version: AcadVersion) -> bool {
        match self {
            ObjectType::AcadProxyObject(_) => { version >= AcadVersion::R2000 },
            ObjectType::DictionaryWithDefault(_) => { version >= AcadVersion::R2000 },
            ObjectType::PlaceHolder(_) => { version >= AcadVersion::R2000 },
            ObjectType::NavisWorksModelDefinition(_) => { version >= AcadVersion::R2018 },
            ObjectType::DataTable(_) => { version >= AcadVersion::R2007 },
            ObjectType::Dictionary(_) => { true },
            ObjectType::DictionaryVariable(_) => { true },
            ObjectType::DimensionAssoc(_) => { version >= AcadVersion::R2004 },
            ObjectType::Field(_) => { version >= AcadVersion::R2004 },
            ObjectType::GeoData(_) => { version >= AcadVersion::R2010 },
            ObjectType::Group(_) => { true },
            ObjectType::IdBuffer(_) => { version >= AcadVersion::R14 },
            ObjectType::ImageDefinition(_) => { version >= AcadVersion::R14 },
            ObjectType::ImageDefinitionReactor(_) => { version >= AcadVersion::R14 },
            ObjectType::LayerFilter(_) => { version >= AcadVersion::R2004 },
            ObjectType::LayerIndex(_) => { version >= AcadVersion::R14 },
            ObjectType::Layout(_) => { version >= AcadVersion::R2000 },
            ObjectType::LightList(_) => { version >= AcadVersion::R2007 },
            ObjectType::Material(_) => { version >= AcadVersion::R2004 },
            ObjectType::MLeaderStyle(_) => { version >= AcadVersion::R2007 },
            ObjectType::MLineStyle(_) => { true },
            ObjectType::ObjectPointer(_) => { version >= AcadVersion::R14 },
            ObjectType::PlotSettings(_) => { version >= AcadVersion::R2000 },
            ObjectType::RapidRTRenderEnvironment(_) => { version >= AcadVersion::R2018 },
            ObjectType::RapidRenderSettings(_) => { version >= AcadVersion::R2018 },
            ObjectType::RasterVariables(_) => { version >= AcadVersion::R14 },
            ObjectType::MentalRayRenderSettings(_) => { version >= AcadVersion::R2007 },
            ObjectType::RenderEnvironment(_) => { version >= AcadVersion::R2007 },
            ObjectType::RenderGlobal(_) => { version >= AcadVersion::R2007 },
            ObjectType::SectionManager(_) => { version >= AcadVersion::R2007 },
            ObjectType::SectionSettings(_) => { version >= AcadVersion::R2007 },
            ObjectType::SortentsTable(_) => { version >= AcadVersion::R14 },
            ObjectType::SpatialFilter(_) => { version >= AcadVersion::R14 },
            ObjectType::SpatialIndex(_) => { version >= AcadVersion::R2000 },
            ObjectType::SunStudy(_) => { version >= AcadVersion::R2013 },
            ObjectType::TableStyle(_) => { version >= AcadVersion::R2004 },
            ObjectType::UnderlayDefinition(_) => { version >= AcadVersion::R2007 },
            ObjectType::VbaProject(_) => { version >= AcadVersion::R2000 },
            ObjectType::VisualStyle(_) => { version >= AcadVersion::R2007 },
            ObjectType::WipeoutVariables(_) => { version >= AcadVersion::R2004 },
            ObjectType::XRecordObject(_) => { version >= AcadVersion::R14 },
        }
    }
    pub(crate) fn from_type_string(type_string: &str) -> Option<ObjectType> {
        match type_string {
            "ACAD_PROXY_OBJECT" => Some(ObjectType::AcadProxyObject(Default::default())),
            "ACDBDICTIONARYWDFLT" => Some(ObjectType::DictionaryWithDefault(Default::default())),
            "ACDBPLACEHOLDER" => Some(ObjectType::PlaceHolder(Default::default())),
            "AcDbNavisworksModelDef" => Some(ObjectType::NavisWorksModelDefinition(Default::default())),
            "DATATABLE" => Some(ObjectType::DataTable(Default::default())),
            "DICTIONARY" => Some(ObjectType::Dictionary(Default::default())),
            "DICTIONARYVAR" => Some(ObjectType::DictionaryVariable(Default::default())),
            "DIMASSOC" => Some(ObjectType::DimensionAssoc(Default::default())),
            "FIELD" => Some(ObjectType::Field(Default::default())),
            "GEODATA" => Some(ObjectType::GeoData(Default::default())),
            "GROUP" => Some(ObjectType::Group(Default::default())),
            "IDBUFFER" => Some(ObjectType::IdBuffer(Default::default())),
            "IMAGEDEF" => Some(ObjectType::ImageDefinition(Default::default())),
            "IMAGEDEF_REACTOR" => Some(ObjectType::ImageDefinitionReactor(Default::default())),
            "LAYER_FILTER" => Some(ObjectType::LayerFilter(Default::default())),
            "LAYER_INDEX" => Some(ObjectType::LayerIndex(Default::default())),
            "LAYOUT" => Some(ObjectType::Layout(Default::default())),
            "LIGHTLIST" => Some(ObjectType::LightList(Default::default())),
            "MATERIAL" => Some(ObjectType::Material(Default::default())),
            "MLEADERSTYLE" => Some(ObjectType::MLeaderStyle(Default::default())),
            "MLINESTYLE" => Some(ObjectType::MLineStyle(Default::default())),
            "OBJECT_PTR" => Some(ObjectType::ObjectPointer(Default::default())),
            "PLOTSETTINGS" => Some(ObjectType::PlotSettings(Default::default())),
            "RAPIDRTRENDERENVIRONMENT" => Some(ObjectType::RapidRTRenderEnvironment(Default::default())),
            "RAPIDRTRENDERSETTINGS" => Some(ObjectType::RapidRenderSettings(Default::default())),
            "RASTERVARIABLES" => Some(ObjectType::RasterVariables(Default::default())),
            "MENTALRAYRENDERSETTINGS" => Some(ObjectType::MentalRayRenderSettings(Default::default())),
            "RENDERENVIRONMENT" => Some(ObjectType::RenderEnvironment(Default::default())),
            "RENDERGLOBAL" => Some(ObjectType::RenderGlobal(Default::default())),
            "SECTIONMANAGER" => Some(ObjectType::SectionManager(Default::default())),
            "SECTIONSETTINGS" => Some(ObjectType::SectionSettings(Default::default())),
            "SORTENTSTABLE" => Some(ObjectType::SortentsTable(Default::default())),
            "SPATIAL_FILTER" => Some(ObjectType::SpatialFilter(Default::default())),
            "SPATIAL_INDEX" => Some(ObjectType::SpatialIndex(Default::default())),
            "SUNSTUDY" => Some(ObjectType::SunStudy(Default::default())),
            "TABLESTYLE" => Some(ObjectType::TableStyle(Default::default())),
            "UNDERLAYDEFINITION" => Some(ObjectType::UnderlayDefinition(Default::default())),
            "VBA_PROJECT" => Some(ObjectType::VbaProject(Default::default())),
            "VISUALSTYLE" => Some(ObjectType::VisualStyle(Default::default())),
            "WIPEOUTVARIABLES" => Some(ObjectType::WipeoutVariables(Default::default())),
            "XRECORD" => Some(ObjectType::XRecordObject(Default::default())),
            _ => None,
        }
    }
    pub(crate) fn to_type_string(&self) -> &str {
        match *self {
            ObjectType::AcadProxyObject(_) => { "ACAD_PROXY_OBJECT" },
            ObjectType::DictionaryWithDefault(_) => { "ACDBDICTIONARYWDFLT" },
            ObjectType::PlaceHolder(_) => { "ACDBPLACEHOLDER" },
            ObjectType::NavisWorksModelDefinition(_) => { "AcDbNavisworksModelDef" },
            ObjectType::DataTable(_) => { "DATATABLE" },
            ObjectType::Dictionary(_) => { "DICTIONARY" },
            ObjectType::DictionaryVariable(_) => { "DICTIONARYVAR" },
            ObjectType::DimensionAssoc(_) => { "DIMASSOC" },
            ObjectType::Field(_) => { "FIELD" },
            ObjectType::GeoData(_) => { "GEODATA" },
            ObjectType::Group(_) => { "GROUP" },
            ObjectType::IdBuffer(_) => { "IDBUFFER" },
            ObjectType::ImageDefinition(_) => { "IMAGEDEF" },
            ObjectType::ImageDefinitionReactor(_) => { "IMAGEDEF_REACTOR" },
            ObjectType::LayerFilter(_) => { "LAYER_FILTER" },
            ObjectType::LayerIndex(_) => { "LAYER_INDEX" },
            ObjectType::Layout(_) => { "LAYOUT" },
            ObjectType::LightList(_) => { "LIGHTLIST" },
            ObjectType::Material(_) => { "MATERIAL" },
            ObjectType::MLeaderStyle(_) => { "MLEADERSTYLE" },
            ObjectType::MLineStyle(_) => { "MLINESTYLE" },
            ObjectType::ObjectPointer(_) => { "OBJECT_PTR" },
            ObjectType::PlotSettings(_) => { "PLOTSETTINGS" },
            ObjectType::RapidRTRenderEnvironment(_) => { "RAPIDRTRENDERENVIRONMENT" },
            ObjectType::RapidRenderSettings(_) => { "RAPIDRTRENDERSETTINGS" },
            ObjectType::RasterVariables(_) => { "RASTERVARIABLES" },
            ObjectType::MentalRayRenderSettings(_) => { "MENTALRAYRENDERSETTINGS" },
            ObjectType::RenderEnvironment(_) => { "RENDERENVIRONMENT" },
            ObjectType::RenderGlobal(_) => { "RENDERGLOBAL" },
            ObjectType::SectionManager(_) => { "SECTIONMANAGER" },
            ObjectType::SectionSettings(_) => { "SECTIONSETTINGS" },
            ObjectType::SortentsTable(_) => { "SORTENTSTABLE" },
            ObjectType::SpatialFilter(_) => { "SPATIAL_FILTER" },
            ObjectType::SpatialIndex(_) => { "SPATIAL_INDEX" },
            ObjectType::SunStudy(_) => { "SUNSTUDY" },
            ObjectType::TableStyle(_) => { "TABLESTYLE" },
            ObjectType::UnderlayDefinition(_) => { "UNDERLAYDEFINITION" },
            ObjectType::VbaProject(_) => { "VBA_PROJECT" },
            ObjectType::VisualStyle(_) => { "VISUALSTYLE" },
            ObjectType::WipeoutVariables(_) => { "WIPEOUTVARIABLES" },
            ObjectType::XRecordObject(_) => { "XRECORD" },
        }
    }
    #[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
    pub(crate) fn try_apply_code_pair(&mut self, pair: &CodePair) -> DxfResult<bool> {
        match *self {
            ObjectType::AcadProxyObject(ref mut obj) => {
                match pair.code {
                    90 => { obj.proxy_object_class_id = pair.assert_i32()?; },
                    91 => { obj.application_object_class_id = pair.assert_i32()?; },
                    93 => { obj.size_in_bits = pair.assert_i32()?; },
                    310 => { obj.binary_object_data.push(pair.assert_binary()?); },
                    330 => { obj.__object_ids_a.push(pair.assert_string()?); },
                    340 => { obj.__object_ids_b.push(pair.assert_string()?); },
                    350 => { obj.__object_ids_c.push(pair.assert_string()?); },
                    360 => { obj.__object_ids_d.push(pair.assert_string()?); },
                    95 => { obj.__object_drawing_format = pair.assert_i32()? as u32; },
                    70 => { obj.is_original_object_format = as_bool(pair.assert_i16()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::DictionaryWithDefault(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::PlaceHolder(ref mut _obj) => {
                return Ok(false);
            },
            ObjectType::NavisWorksModelDefinition(ref mut obj) => {
                match pair.code {
                    1 => { obj.model_path = pair.assert_string()?; },
                    290 => { obj.is_model_loaded_on_drawing_open = pair.assert_bool()?; },
                    10 => { obj.minimum_extent.x = pair.assert_f64()?; },
                    20 => { obj.minimum_extent.y = pair.assert_f64()?; },
                    30 => { obj.minimum_extent.z = pair.assert_f64()?; },
                    11 => { obj.maximum_extent.x = pair.assert_f64()?; },
                    21 => { obj.maximum_extent.y = pair.assert_f64()?; },
                    31 => { obj.maximum_extent.z = pair.assert_f64()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::DataTable(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::Dictionary(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::DictionaryVariable(ref mut obj) => {
                match pair.code {
                    280 => { obj.object_schema_number = pair.assert_i16()?; },
                    1 => { obj.value = pair.assert_string()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::DimensionAssoc(ref mut obj) => {
                match pair.code {
                    330 => { obj.__dimension_handle = pair.as_handle()?; },
                    90 => { obj.associativity_flags = pair.assert_i32()?; },
                    70 => { obj.is_trans_space = as_bool(pair.assert_i16()?); },
                    71 => { obj.rotated_dimension_type = enum_from_number!(RotatedDimensionType, Parallel, from_i16, pair.assert_i16()?); },
                    1 => { obj.class_name = pair.assert_string()?; },
                    72 => { obj.object_osnap_type = enum_from_number!(ObjectOsnapType, None, from_i16, pair.assert_i16()?); },
                    331 => { obj.__main_object_handle = pair.as_handle()?; },
                    73 => { obj.main_object_subentity_type = enum_from_number!(SubentityType, Edge, from_i16, pair.assert_i16()?); },
                    91 => { obj.main_object_gs_marker_index = pair.assert_i32()?; },
                    301 => { obj.__main_object_xref_handle = pair.as_handle()?; },
                    40 => { obj.near_osnap_geometry_parameter = pair.assert_f64()?; },
                    10 => { obj.osnap_point.x = pair.assert_f64()?; },
                    20 => { obj.osnap_point.y = pair.assert_f64()?; },
                    30 => { obj.osnap_point.z = pair.assert_f64()?; },
                    332 => { obj.__intersection_object_handle = pair.as_handle()?; },
                    74 => { obj.intersection_subentity_type = enum_from_number!(SubentityType, Edge, from_i16, pair.assert_i16()?); },
                    92 => { obj.intersection_object_gs_marker_index = pair.assert_i32()?; },
                    302 => { obj.__insertion_object_xref_handle = pair.as_handle()?; },
                    75 => { obj.has_last_point_reference = as_bool(pair.assert_i16()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::Field(ref mut obj) => {
                match pair.code {
                    1 => { obj.evaluator_id = pair.assert_string()?; },
                    2 => { obj.field_code_string = pair.assert_string()?; },
                    3 => { obj.field_code_string_overflow = pair.assert_string()?; },
                    4 => { obj.__format_string = pair.assert_string()?; },
                    300 => { obj.evaluation_error_message = pair.assert_string()?; },
                    90 => { obj.__child_field_count = pair.assert_i32()?; },
                    360 => { obj.__child_fields_handle.push(pair.as_handle()?); },
                    91 => { obj.evaluation_option = pair.assert_i32()?; },
                    92 => { obj.filling_option = pair.assert_i32()?; },
                    94 => { obj.field_state = pair.assert_i32()?; },
                    95 => { obj.evaluation_status = pair.assert_i32()?; },
                    96 => { obj.evaluation_error_code = pair.assert_i32()?; },
                    97 => { obj.__object_id_count = pair.assert_i32()?; },
                    331 => { obj.__objects_handle.push(pair.as_handle()?); },
                    93 => { obj.__data_set_count = pair.assert_i32()?; },
                    6 => { obj.field_data_keys.push(pair.assert_string()?); },
                    7 => { obj.evaluated_cache_key = pair.assert_string()?; },
                    140 => { obj.__double_value = pair.assert_f64()?; },
                    330 => { obj.__id_value = pair.as_handle()?; },
                    310 => { obj.__binary_data = pair.assert_binary()?; },
                    301 => { obj.__format_string_code301 = pair.assert_string()?; },
                    9 => { obj.__format_string_overflow = pair.assert_string()?; },
                    98 => { obj.__format_string_length = pair.assert_i32()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::GeoData(ref mut obj) => {
                match pair.code {
                    90 => { obj.version = enum_from_number!(GeoDataVersion, R2009, from_i32, pair.assert_i32()?); },
                    70 => { obj.coordinate_type = enum_from_number!(DesignCoordinateType, Unknown, from_i16, pair.assert_i16()?); },
                    10 => { obj.design_point.x = pair.assert_f64()?; },
                    20 => { obj.design_point.y = pair.assert_f64()?; },
                    30 => { obj.design_point.z = pair.assert_f64()?; },
                    11 => { obj.reference_point.x = pair.assert_f64()?; },
                    21 => { obj.reference_point.y = pair.assert_f64()?; },
                    31 => { obj.reference_point.z = pair.assert_f64()?; },
                    12 => { obj.north_vector.x = pair.assert_f64()?; },
                    22 => { obj.north_vector.y = pair.assert_f64()?; },
                    40 => { obj.horizontal_unit_scale = pair.assert_f64()?; },
                    41 => { obj.vertical_unit_scale = pair.assert_f64()?; },
                    91 => { obj.horizontal_units = enum_from_number!(Units, Unitless, from_i32, pair.assert_i32()?); },
                    92 => { obj.vertical_units = enum_from_number!(Units, Unitless, from_i32, pair.assert_i32()?); },
                    210 => { obj.up_direction.x = pair.assert_f64()?; },
                    220 => { obj.up_direction.y = pair.assert_f64()?; },
                    230 => { obj.up_direction.z = pair.assert_f64()?; },
                    95 => { obj.scale_estimation_method = enum_from_number!(ScaleEstimationMethod, None, from_i32, pair.assert_i32()?); },
                    294 => { obj.use_sea_level_correction = pair.assert_bool()?; },
                    141 => { obj.user_specified_scale_factor = pair.assert_f64()?; },
                    142 => { obj.sea_level_elevation = pair.assert_f64()?; },
                    143 => { obj.coordinate_projection_radius = pair.assert_f64()?; },
                    301 => { obj.coordinate_system_definition = pair.assert_string()?; },
                    302 => { obj.geo_rss_tag = pair.assert_string()?; },
                    305 => { obj.observation_from_tag = pair.assert_string()?; },
                    306 => { obj.observation_to_tag = pair.assert_string()?; },
                    307 => { obj.observation_coverage_tag = pair.assert_string()?; },
                    93 => { obj.__geo_mesh_point_count = pair.assert_i32()?; },
                    13 => { obj.__source_mesh_x_points.push(pair.assert_f64()?); },
                    23 => { obj.__source_mesh_y_points.push(pair.assert_f64()?); },
                    14 => { obj.__destination_mesh_x_points.push(pair.assert_f64()?); },
                    24 => { obj.__destination_mesh_y_points.push(pair.assert_f64()?); },
                    96 => { obj.__faces_count = pair.assert_i32()?; },
                    97 => { obj.__face_point_index_x.push(f64::from(pair.assert_i32()?)); },
                    98 => { obj.__face_point_index_y.push(f64::from(pair.assert_i32()?)); },
                    99 => { obj.__face_point_index_z.push(f64::from(pair.assert_i32()?)); },
                    _ => return Ok(false),
                }
            },
            ObjectType::Group(ref mut obj) => {
                match pair.code {
                    300 => { obj.description = pair.assert_string()?; },
                    70 => { obj.is_named = !as_bool(pair.assert_i16()?); },
                    71 => { obj.is_selectable = as_bool(pair.assert_i16()?); },
                    340 => { obj.__entities_handle.push(pair.as_handle()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::IdBuffer(ref mut obj) => {
                match pair.code {
                    330 => { obj.__entities_handle.push(pair.as_handle()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::ImageDefinition(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version = pair.assert_i32()?; },
                    1 => { obj.file_path = pair.assert_string()?; },
                    10 => { obj.image_width = pair.assert_f64()? as i32; },
                    20 => { obj.image_height = pair.assert_f64()? as i32; },
                    11 => { obj.pixel_width = pair.assert_f64()?; },
                    12 => { obj.pixel_height = pair.assert_f64()?; },
                    280 => { obj.is_image_loaded = as_bool(pair.assert_i16()?); },
                    281 => { obj.resolution_units = enum_from_number!(ImageResolutionUnits, NoUnits, from_i16, pair.assert_i16()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::ImageDefinitionReactor(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version = pair.assert_i32()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::LayerFilter(ref mut obj) => {
                match pair.code {
                    8 => { obj.layer_names.push(pair.assert_string()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::LayerIndex(ref mut obj) => {
                match pair.code {
                    40 => { obj.time_stamp = as_datetime_local(pair.assert_f64()?); },
                    8 => { obj.layer_names.push(pair.assert_string()?); },
                    360 => { obj.__id_buffers_handle.push(pair.as_handle()?); },
                    90 => { obj.id_buffer_counts.push(pair.assert_i32()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::Layout(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::LightList(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::Material(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::MLeaderStyle(ref mut obj) => {
                match pair.code {
                    170 => { obj.content_type = pair.assert_i16()?; },
                    171 => { obj.draw_m_leader_order_type = pair.assert_i16()?; },
                    172 => { obj.draw_leader_order_type = pair.assert_i16()?; },
                    90 => { obj.max_leader_segment_count = pair.assert_i32()?; },
                    40 => { obj.first_segment_angle_constraint = pair.assert_f64()?; },
                    41 => { obj.second_segment_angle_constraint = pair.assert_f64()?; },
                    173 => { obj.leader_line_type = pair.assert_i16()?; },
                    91 => { obj.leader_line_color = pair.assert_i32()?; },
                    340 => { obj.__line_leader_type_handle = pair.as_handle()?; },
                    92 => { obj.leader_line_weight = pair.assert_i32()?; },
                    290 => { obj.enable_landing = pair.assert_bool()?; },
                    42 => { obj.landing_gap = pair.assert_f64()?; },
                    291 => { obj.enable_dogleg = pair.assert_bool()?; },
                    43 => { obj.dogleg_length = pair.assert_f64()?; },
                    3 => { obj.m_leader_style_description = pair.assert_string()?; },
                    341 => { obj.__arrowhead_handle = pair.as_handle()?; },
                    44 => { obj.arrowhead_size = pair.assert_f64()?; },
                    300 => { obj.default_m_text_contents = pair.assert_string()?; },
                    342 => { obj.__m_text_style_handle = pair.as_handle()?; },
                    174 => { obj.text_left_attachment_type = pair.assert_i16()?; },
                    175 => { obj.text_angle_type = pair.assert_i16()?; },
                    176 => { obj.text_alignment_type = pair.assert_i16()?; },
                    178 => { obj.text_right_attachment_type = pair.assert_i16()?; },
                    93 => { obj.text_color = pair.assert_i32()?; },
                    45 => { obj.text_height = pair.assert_f64()?; },
                    292 => { obj.enable_frame_text = pair.assert_bool()?; },
                    297 => { obj.always_align_text_left = pair.assert_bool()?; },
                    46 => { obj.align_gap = pair.assert_f64()?; },
                    343 => { obj.__block_content_handle = pair.as_handle()?; },
                    94 => { obj.block_content_color = pair.assert_i32()?; },
                    47 => { obj.block_content_x_scale = pair.assert_f64()?; },
                    49 => { obj.block_content_y_scale = pair.assert_f64()?; },
                    140 => { obj.block_content_z_scale = pair.assert_f64()?; },
                    293 => { obj.enable_block_content_scale = pair.assert_bool()?; },
                    141 => { obj.block_content_rotation = pair.assert_f64()?; },
                    294 => { obj.enable_block_content_rotation = pair.assert_bool()?; },
                    177 => { obj.block_content_connection_type = pair.assert_i16()?; },
                    142 => { obj.scale = pair.assert_f64()?; },
                    295 => { obj.overwrite_field_value = pair.assert_bool()?; },
                    296 => { obj.is_annotative = pair.assert_bool()?; },
                    143 => { obj.break_gap_size = pair.assert_f64()?; },
                    271 => { obj.text_attachment_direction = enum_from_number!(TextAttachmentDirection, Horizontal, from_i16, pair.assert_i16()?); },
                    272 => { obj.bottom_text_attachment_direction = enum_from_number!(BottomTextAttachmentDirection, Center, from_i16, pair.assert_i16()?); },
                    273 => { obj.top_text_attachment_direction = enum_from_number!(TopTextAttachmentDirection, Center, from_i16, pair.assert_i16()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::MLineStyle(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::ObjectPointer(ref mut _obj) => {
                return Ok(false);
            },
            ObjectType::PlotSettings(ref mut obj) => {
                match pair.code {
                    1 => { obj.page_setup_name = pair.assert_string()?; },
                    2 => { obj.printer_name = pair.assert_string()?; },
                    4 => { obj.paper_size = pair.assert_string()?; },
                    6 => { obj.plot_view_name = pair.assert_string()?; },
                    40 => { obj.unprintable_left_margin_size = pair.assert_f64()?; },
                    41 => { obj.unprintable_bottom_margin_size = pair.assert_f64()?; },
                    42 => { obj.unprintable_right_margin_size = pair.assert_f64()?; },
                    43 => { obj.unprintable_top_margin_size = pair.assert_f64()?; },
                    44 => { obj.paper_width = pair.assert_f64()?; },
                    45 => { obj.paper_height = pair.assert_f64()?; },
                    46 => { obj.plot_origin_x_value = pair.assert_f64()?; },
                    47 => { obj.plot_origin_y_value = pair.assert_f64()?; },
                    48 => { obj.plot_window_lower_left_x_value = pair.assert_f64()?; },
                    49 => { obj.plot_window_lower_left_y_value = pair.assert_f64()?; },
                    140 => { obj.plot_window_upper_right_x_value = pair.assert_f64()?; },
                    141 => { obj.plot_window_upper_right_y_value = pair.assert_f64()?; },
                    142 => { obj.custom_print_scale_numerator = pair.assert_f64()?; },
                    143 => { obj.custom_print_scale_denominator = pair.assert_f64()?; },
                    70 => { obj.plot_layout_flags = i32::from(pair.assert_i16()?); },
                    72 => { obj.plot_paper_units = enum_from_number!(PlotPaperUnits, Inches, from_i16, pair.assert_i16()?); },
                    73 => { obj.plot_rotation = enum_from_number!(PlotRotation, NoRotation, from_i16, pair.assert_i16()?); },
                    74 => { obj.plot_type = enum_from_number!(PlotType, DrawingExtents, from_i16, pair.assert_i16()?); },
                    7 => { obj.current_style_sheet = pair.assert_string()?; },
                    75 => { obj.standard_scale = enum_from_number!(StandardScale, ScaledToFit, from_i16, pair.assert_i16()?); },
                    76 => { obj.shade_plot_mode = enum_from_number!(ShadePlotMode, AsDisplayed, from_i16, pair.assert_i16()?); },
                    77 => { obj.shade_plot_resolution_level = enum_from_number!(ShadePlotResolutionLevel, Normal, from_i16, pair.assert_i16()?); },
                    78 => { obj.shade_plot_custom_dpi = i32::from(pair.assert_i16()?); },
                    147 => { obj.standard_scale_value = pair.assert_f64()?; },
                    148 => { obj.paper_image_origin_x = pair.assert_f64()?; },
                    149 => { obj.paper_image_origin_y = pair.assert_f64()?; },
                    333 => { obj.__shade_plot_object_handle = pair.as_handle()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::RapidRTRenderEnvironment(ref mut obj) => {
                match pair.code {
                    90 => { obj.subclass_version = pair.assert_i32()?; },
                    290 => { obj.is_enabled = pair.assert_bool()?; },
                    1 => { obj.image_based_lignting_map_file_name = pair.assert_string()?; },
                    40 => { obj.rotation_angle = pair.assert_f64()?; },
                    340 => { obj.__custom_background_handle = pair.as_handle()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::RapidRenderSettings(ref mut obj) => {
                match pair.code {
                    90 => { obj.subclass_version = pair.assert_i32()?; },
                    1 => { obj.render_preset_name = pair.assert_string()?; },
                    290 => { obj.are_materials_enabled = pair.assert_bool()?; },
                    70 => { obj.render_duration = enum_from_number!(RenderDuration, ByTime, from_i16, pair.assert_i16()?); },
                    40 => { obj.filter_width = pair.assert_f64()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::RasterVariables(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version = pair.assert_i32()?; },
                    70 => { obj.is_display_frame_image = as_bool(pair.assert_i16()?); },
                    71 => { obj.is_high_display_quality = as_bool(pair.assert_i16()?); },
                    72 => { obj.image_units = enum_from_number!(RasterImageUnits, None, from_i16, pair.assert_i16()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::MentalRayRenderSettings(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version_1 = pair.assert_i32()?; },
                    1 => { obj.preset_name = pair.assert_string()?; },
                    290 => { obj.render_materials = pair.assert_bool()?; },
                    70 => { obj.sampling_filter_type = enum_from_number!(SamplingFilterType, Box, from_i16, pair.assert_i16()?); },
                    40 => { obj.filter_width = pair.assert_f64()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::RenderEnvironment(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version = pair.assert_i32()?; },
                    290 => { obj.is_fog_enabled = pair.assert_bool()?; },
                    280 => { obj.fog_color_red = i32::from(pair.assert_i16()?); },
                    40 => { obj.near_fog_density_percent = pair.assert_f64()?; },
                    1 => { obj.environment_image_file_name = pair.assert_string()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::RenderGlobal(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version = pair.assert_i32()?; },
                    290 => { obj.save_to_file = pair.assert_bool()?; },
                    1 => { obj.save_to_file_name = pair.assert_string()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::SectionManager(ref mut obj) => {
                match pair.code {
                    70 => { obj.requires_full_update = as_bool(pair.assert_i16()?); },
                    90 => { obj.__section_count = pair.assert_i32()?; },
                    330 => { obj.__section_entities_handle.push(pair.as_handle()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::SectionSettings(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::SortentsTable(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::SpatialFilter(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::SpatialIndex(ref mut obj) => {
                match pair.code {
                    40 => { obj.timestamp = as_datetime_local(pair.assert_f64()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::SunStudy(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::TableStyle(_) => { panic!("this case should have been covered in a custom reader"); },
            ObjectType::UnderlayDefinition(ref mut obj) => {
                match pair.code {
                    1 => { obj.file_name = pair.assert_string()?; },
                    2 => { obj.name = pair.assert_string()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::VbaProject(ref mut obj) => {
                match pair.code {
                    310 => { obj.__hex_data.push(pair.assert_binary()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::VisualStyle(ref mut obj) => {
                match pair.code {
                    2 => { obj.description = pair.assert_string()?; },
                    70 => { obj.type_code = pair.assert_i16()?; },
                    71 => { obj.face_lighting_model = enum_from_number!(FaceLightingModel, Visible, from_i16, pair.assert_i16()?); },
                    72 => { obj.face_lighting_quality = enum_from_number!(FaceLightingQuality, PerFace, from_i16, pair.assert_i16()?); },
                    73 => { obj.face_color_mode = enum_from_number!(FaceColorMode, ObjectColor, from_i16, pair.assert_i16()?); },
                    90 => { obj.face_modifier = enum_from_number!(FaceModifier, None, from_i32, pair.assert_i32()?); },
                    40 => { obj.face_opacity_level = pair.assert_f64()?; },
                    41 => { obj.face_specular_level = pair.assert_f64()?; },
                    62 => { obj.color1 = Color::from_raw_value(pair.assert_i16()?); },
                    63 => { obj.color2 = Color::from_raw_value(pair.assert_i16()?); },
                    421 => { obj.face_style_mono_color = pair.assert_i32()?; },
                    74 => { obj.edge_style_model = enum_from_number!(EdgeStyleModel, NoEdges, from_i16, pair.assert_i16()?); },
                    91 => { obj.edge_style = pair.assert_i32()?; },
                    64 => { obj.edge_intersection_color = Color::from_raw_value(pair.assert_i16()?); },
                    65 => { obj.edge_obscured_color = Color::from_raw_value(pair.assert_i16()?); },
                    75 => { obj.edge_obscured_line_type = pair.assert_i16()?; },
                    175 => { obj.edge_intersection_line_type = pair.assert_i16()?; },
                    42 => { obj.edge_crease_angle = pair.assert_f64()?; },
                    92 => { obj.edge_modifiers = pair.assert_i32()?; },
                    66 => { obj.edge_color = Color::from_raw_value(pair.assert_i16()?); },
                    43 => { obj.edge_opacity_level = pair.assert_f64()?; },
                    76 => { obj.edge_width = pair.assert_i16()?; },
                    77 => { obj.edge_overhang = pair.assert_i16()?; },
                    78 => { obj.edge_jitter = pair.assert_i16()?; },
                    67 => { obj.edge_silhouette_color = Color::from_raw_value(pair.assert_i16()?); },
                    79 => { obj.edge_silhouette_width = pair.assert_i16()?; },
                    170 => { obj.edge_halo_gap = pair.assert_i16()?; },
                    171 => { obj.edge_iso_line_count = i32::from(pair.assert_i16()?); },
                    290 => { obj.hide_edge_line_precision = pair.assert_bool()?; },
                    174 => { obj.edge_style_apply_flags = i32::from(pair.assert_i16()?); },
                    93 => { obj.display_style_settings = pair.assert_i32()?; },
                    44 => { obj.brightness = pair.assert_f64()?; },
                    173 => { obj.shadow_type = pair.assert_i16()?; },
                    291 => { obj.internal_flag = pair.assert_bool()?; },
                    _ => return Ok(false),
                }
            },
            ObjectType::WipeoutVariables(ref mut obj) => {
                match pair.code {
                    90 => { obj.class_version = pair.assert_i32()?; },
                    70 => { obj.display_image_frame = as_bool(pair.assert_i16()?); },
                    _ => return Ok(false),
                }
            },
            ObjectType::XRecordObject(_) => { panic!("this case should have been covered in a custom reader"); },
        }
        Ok(true)
    }

    #[allow(clippy::cognitive_complexity)] // long function, no good way to simplify this
    pub(crate) fn add_code_pairs(&self, pairs: &mut Vec<CodePair>, version: AcadVersion) {
        match *self {
            ObjectType::AcadProxyObject(ref obj) => {
                if version >= AcadVersion::R13 {
                    pairs.push(CodePair::new_string(100, &String::from("AcDbProxyObject")));
                }
                pairs.push(CodePair::new_i32(90, obj.proxy_object_class_id));
                pairs.push(CodePair::new_i32(91, obj.application_object_class_id));
                pairs.push(CodePair::new_i32(93, obj.size_in_bits));
                for v in &obj.binary_object_data {
                    pairs.push(CodePair::new_binary(310, v.clone()));
                }
                for item in &obj.object_ids {
                    pairs.push(CodePair::new_string(330, item));
                }
                pairs.push(CodePair::new_i32(94, 0));
                pairs.push(CodePair::new_i32(95, obj.__object_drawing_format as i32));
                pairs.push(CodePair::new_i16(70, as_i16(obj.is_original_object_format)));
            },
            ObjectType::DictionaryWithDefault(_) => { panic!("this case should have been covered in a custom writer"); },
            ObjectType::PlaceHolder(ref _obj) => {
            },
            ObjectType::NavisWorksModelDefinition(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbNavisworksModelDef"));
                pairs.push(CodePair::new_string(1, &obj.model_path));
                pairs.push(CodePair::new_bool(290, obj.is_model_loaded_on_drawing_open));
                pairs.push(CodePair::new_f64(10, obj.minimum_extent.x));
                pairs.push(CodePair::new_f64(20, obj.minimum_extent.y));
                pairs.push(CodePair::new_f64(30, obj.minimum_extent.z));
                pairs.push(CodePair::new_f64(11, obj.maximum_extent.x));
                pairs.push(CodePair::new_f64(21, obj.maximum_extent.y));
                pairs.push(CodePair::new_f64(31, obj.maximum_extent.z));
                pairs.push(CodePair::new_bool(290, obj.is_host_geometry_drawn));
            },
            ObjectType::DataTable(_) => { panic!("this case should have been covered in a custom writer"); },
            ObjectType::Dictionary(_) => { panic!("this case should have been covered in a custom writer"); },
            ObjectType::DictionaryVariable(ref obj) => {
                pairs.push(CodePair::new_str(100, "DictionaryVariables"));
                pairs.push(CodePair::new_i16(280, obj.object_schema_number));
                pairs.push(CodePair::new_string(1, &obj.value));
            },
            ObjectType::DimensionAssoc(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbDimAssoc"));
                pairs.push(CodePair::new_string(330, &obj.__dimension_handle.as_string()));
                pairs.push(CodePair::new_i32(90, obj.associativity_flags));
                pairs.push(CodePair::new_i16(70, as_i16(obj.is_trans_space)));
                pairs.push(CodePair::new_i16(71, obj.rotated_dimension_type as i16));
                pairs.push(CodePair::new_string(1, &obj.class_name));
                pairs.push(CodePair::new_i16(72, obj.object_osnap_type as i16));
                pairs.push(CodePair::new_string(331, &obj.__main_object_handle.as_string()));
                pairs.push(CodePair::new_i16(73, obj.main_object_subentity_type as i16));
                pairs.push(CodePair::new_i32(91, obj.main_object_gs_marker_index));
                pairs.push(CodePair::new_string(301, &obj.__main_object_xref_handle.as_string()));
                pairs.push(CodePair::new_f64(40, obj.near_osnap_geometry_parameter));
                pairs.push(CodePair::new_f64(10, obj.osnap_point.x));
                pairs.push(CodePair::new_f64(20, obj.osnap_point.y));
                pairs.push(CodePair::new_f64(30, obj.osnap_point.z));
                pairs.push(CodePair::new_string(332, &obj.__intersection_object_handle.as_string()));
                pairs.push(CodePair::new_i16(74, obj.intersection_subentity_type as i16));
                pairs.push(CodePair::new_i32(92, obj.intersection_object_gs_marker_index));
                pairs.push(CodePair::new_string(302, &obj.__insertion_object_xref_handle.as_string()));
                pairs.push(CodePair::new_i16(75, as_i16(obj.has_last_point_reference)));
            },
            ObjectType::Field(ref obj) => {
                pairs.push(CodePair::new_string(1, &obj.evaluator_id));
                pairs.push(CodePair::new_string(2, &obj.field_code_string));
                pairs.push(CodePair::new_string(3, &obj.field_code_string_overflow));
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_string(4, &obj.__format_string));
                }
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_string(300, &obj.evaluation_error_message));
                }
                pairs.push(CodePair::new_i32(90, obj.__child_fields_handle.len() as i32));
                for v in &obj.__child_fields_handle {
                    pairs.push(CodePair::new_string(360, &v.as_string()));
                }
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i32(91, obj.evaluation_option));
                }
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i32(92, obj.filling_option));
                }
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i32(94, obj.field_state));
                }
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i32(95, obj.evaluation_status));
                }
                if version <= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i32(96, obj.evaluation_error_code));
                }
                pairs.push(CodePair::new_i32(97, obj.__objects_handle.len() as i32));
                for item in &obj.__objects_handle {
                    if version <= AcadVersion::R2007 {
                        pairs.push(CodePair::new_string(330, &item.as_string()));
                    }
                    if version >= AcadVersion::R2010 {
                        pairs.push(CodePair::new_string(331, &item.as_string()));
                    }
                }
                pairs.push(CodePair::new_i32(93, obj.field_data_keys.len() as i32));
                for v in &obj.field_data_keys {
                    pairs.push(CodePair::new_string(6, v));
                }
                pairs.push(CodePair::new_string(7, &obj.evaluated_cache_key));
                pairs.push(CodePair::new_i32(90, obj.__value_type_code));
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_string(301, &obj.__format_string));
                }
                pairs.push(CodePair::new_string(9, &obj.__format_string_overflow));
                pairs.push(CodePair::new_i32(98, obj.__format_string.len() as i32));
            },
            ObjectType::GeoData(ref obj) => {
                pairs.push(CodePair::new_i32(90, obj.version as i32));
                pairs.push(CodePair::new_i16(70, obj.coordinate_type as i16));
                pairs.push(CodePair::new_f64(10, obj.design_point.x));
                pairs.push(CodePair::new_f64(20, obj.design_point.y));
                pairs.push(CodePair::new_f64(30, obj.design_point.z));
                pairs.push(CodePair::new_f64(11, obj.reference_point.x));
                pairs.push(CodePair::new_f64(21, obj.reference_point.y));
                pairs.push(CodePair::new_f64(31, obj.reference_point.z));
                pairs.push(CodePair::new_f64(12, obj.north_vector.x));
                pairs.push(CodePair::new_f64(22, obj.north_vector.y));
                pairs.push(CodePair::new_f64(40, obj.horizontal_unit_scale));
                pairs.push(CodePair::new_f64(41, obj.vertical_unit_scale));
                pairs.push(CodePair::new_i32(91, obj.horizontal_units as i32));
                pairs.push(CodePair::new_i32(92, obj.vertical_units as i32));
                pairs.push(CodePair::new_f64(210, obj.up_direction.x));
                pairs.push(CodePair::new_f64(220, obj.up_direction.y));
                pairs.push(CodePair::new_f64(230, obj.up_direction.z));
                pairs.push(CodePair::new_i32(95, obj.scale_estimation_method as i32));
                pairs.push(CodePair::new_bool(294, obj.use_sea_level_correction));
                pairs.push(CodePair::new_f64(141, obj.user_specified_scale_factor));
                pairs.push(CodePair::new_f64(142, obj.sea_level_elevation));
                pairs.push(CodePair::new_f64(143, obj.coordinate_projection_radius));
                pairs.push(CodePair::new_string(301, &obj.coordinate_system_definition));
                pairs.push(CodePair::new_string(302, &obj.geo_rss_tag));
                pairs.push(CodePair::new_string(305, &obj.observation_from_tag));
                pairs.push(CodePair::new_string(306, &obj.observation_to_tag));
                pairs.push(CodePair::new_string(307, &obj.observation_coverage_tag));
                pairs.push(CodePair::new_i32(93, obj.geo_mesh_points.len() as i32));
                for item in &obj.geo_mesh_points {
                    pairs.push(CodePair::new_f64(13, item.source.x));
                    pairs.push(CodePair::new_f64(23, item.source.y));
                    pairs.push(CodePair::new_f64(14, item.destination.x));
                    pairs.push(CodePair::new_f64(24, item.destination.y));
                }
                pairs.push(CodePair::new_i32(96, obj.face_indices.len() as i32));
                for item in &obj.face_indices {
                    pairs.push(CodePair::new_i32(97, item.x as i32));
                    pairs.push(CodePair::new_i32(98, item.y as i32));
                    pairs.push(CodePair::new_i32(99, item.z as i32));
                }
            },
            ObjectType::Group(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbGroup"));
                pairs.push(CodePair::new_string(300, &obj.description));
                pairs.push(CodePair::new_i16(70, as_i16(!obj.is_named)));
                pairs.push(CodePair::new_i16(71, as_i16(obj.is_selectable)));
                for v in &obj.__entities_handle {
                    pairs.push(CodePair::new_string(340, &v.as_string()));
                }
            },
            ObjectType::IdBuffer(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbIdBuffer"));
                for v in &obj.__entities_handle {
                    pairs.push(CodePair::new_string(330, &v.as_string()));
                }
            },
            ObjectType::ImageDefinition(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbRasterImageDef"));
                pairs.push(CodePair::new_i32(90, obj.class_version));
                pairs.push(CodePair::new_string(1, &obj.file_path));
                pairs.push(CodePair::new_f64(10, f64::from(obj.image_width)));
                pairs.push(CodePair::new_f64(20, f64::from(obj.image_height)));
                pairs.push(CodePair::new_f64(11, obj.pixel_width));
                pairs.push(CodePair::new_f64(12, obj.pixel_height));
                pairs.push(CodePair::new_i16(280, as_i16(obj.is_image_loaded)));
                pairs.push(CodePair::new_i16(281, obj.resolution_units as i16));
            },
            ObjectType::ImageDefinitionReactor(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbRasterImageDefReactor"));
                pairs.push(CodePair::new_i32(90, obj.class_version));
            },
            ObjectType::LayerFilter(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbFilter")));
                pairs.push(CodePair::new_string(100, &String::from("AcDbLayerFilter")));
                for v in &obj.layer_names {
                    pairs.push(CodePair::new_string(8, v));
                }
            },
            ObjectType::LayerIndex(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbIndex")));
                pairs.push(CodePair::new_f64(40, as_double_local(obj.time_stamp)));
                pairs.push(CodePair::new_string(100, &String::from("AcDbLayerIndex")));
                for v in &obj.layer_names {
                    pairs.push(CodePair::new_string(8, v));
                }
                for v in &obj.__id_buffers_handle {
                    pairs.push(CodePair::new_string(360, &v.as_string()));
                }
                for v in &obj.id_buffer_counts {
                    pairs.push(CodePair::new_i32(90, *v));
                }
            },
            ObjectType::Layout(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbLayout"));
                pairs.push(CodePair::new_string(1, &obj.layout_name));
                pairs.push(CodePair::new_i16(70, obj.layout_flags as i16));
                pairs.push(CodePair::new_i16(71, obj.tab_order as i16));
                pairs.push(CodePair::new_f64(10, obj.minimum_limits.x));
                pairs.push(CodePair::new_f64(20, obj.minimum_limits.y));
                pairs.push(CodePair::new_f64(11, obj.maximum_limits.x));
                pairs.push(CodePair::new_f64(21, obj.maximum_limits.y));
                pairs.push(CodePair::new_f64(12, obj.insertion_base_point.x));
                pairs.push(CodePair::new_f64(22, obj.insertion_base_point.y));
                pairs.push(CodePair::new_f64(32, obj.insertion_base_point.z));
                pairs.push(CodePair::new_f64(14, obj.minimum_extents.x));
                pairs.push(CodePair::new_f64(24, obj.minimum_extents.y));
                pairs.push(CodePair::new_f64(34, obj.minimum_extents.z));
                pairs.push(CodePair::new_f64(15, obj.maximum_extents.x));
                pairs.push(CodePair::new_f64(25, obj.maximum_extents.y));
                pairs.push(CodePair::new_f64(35, obj.maximum_extents.z));
                pairs.push(CodePair::new_f64(146, obj.elevation));
                pairs.push(CodePair::new_f64(13, obj.ucs_origin.x));
                pairs.push(CodePair::new_f64(23, obj.ucs_origin.y));
                pairs.push(CodePair::new_f64(33, obj.ucs_origin.z));
                pairs.push(CodePair::new_f64(16, obj.ucs_x_axis.x));
                pairs.push(CodePair::new_f64(26, obj.ucs_x_axis.y));
                pairs.push(CodePair::new_f64(36, obj.ucs_x_axis.z));
                pairs.push(CodePair::new_f64(17, obj.ucs_y_axis.x));
                pairs.push(CodePair::new_f64(27, obj.ucs_y_axis.y));
                pairs.push(CodePair::new_f64(37, obj.ucs_y_axis.z));
                pairs.push(CodePair::new_i16(76, obj.ucs_orthographic_type as i16));
                pairs.push(CodePair::new_string(330, &obj.__viewport_handle.as_string()));
                pairs.push(CodePair::new_string(345, &obj.__table_record_handle.as_string()));
                pairs.push(CodePair::new_string(346, &obj.__table_record_base_handle.as_string()));
            },
            ObjectType::LightList(_) => { panic!("this case should have been covered in a custom writer"); },
            ObjectType::Material(ref obj) => {
                pairs.push(CodePair::new_string(1, &obj.name));
                pairs.push(CodePair::new_string(2, &obj.description));
                pairs.push(CodePair::new_i16(70, as_i16(obj.override_ambient_color)));
                pairs.push(CodePair::new_f64(40, obj.ambient_color_factor));
                pairs.push(CodePair::new_i32(90, obj.ambient_color_value));
                pairs.push(CodePair::new_i16(71, as_i16(obj.override_diffuse_color)));
                pairs.push(CodePair::new_f64(41, obj.diffuse_color_factor));
                pairs.push(CodePair::new_i32(91, obj.diffuse_color_value));
                pairs.push(CodePair::new_f64(42, obj.diffuse_map_blend_factor));
                pairs.push(CodePair::new_i16(72, as_i16(obj.use_image_file_for_diffuse_map)));
                pairs.push(CodePair::new_string(3, &obj.diffuse_map_file_name));
                pairs.push(CodePair::new_i16(73, obj.diffuse_map_projection_method as i16));
                pairs.push(CodePair::new_i16(74, obj.diffuse_map_tiling_method as i16));
                pairs.push(CodePair::new_i16(75, obj.diffuse_map_auto_transform_method as i16));
                for item in &obj.diffuse_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(43, *item));
                }
                pairs.push(CodePair::new_f64(45, obj.specular_color_factor));
                pairs.push(CodePair::new_i16(76, as_i16(obj.override_specular_color)));
                pairs.push(CodePair::new_f64(45, obj.specular_color_factor));
                pairs.push(CodePair::new_i32(92, obj.specular_color_value));
                pairs.push(CodePair::new_f64(46, obj.specular_map_blend_factor));
                pairs.push(CodePair::new_i16(77, as_i16(obj.use_image_file_for_specular_map)));
                pairs.push(CodePair::new_string(4, &obj.specular_map_file_name));
                pairs.push(CodePair::new_i16(78, obj.specular_map_projection_method as i16));
                pairs.push(CodePair::new_i16(79, obj.specular_map_tiling_method as i16));
                pairs.push(CodePair::new_i16(170, obj.specular_map_auto_transform_method as i16));
                for item in &obj.specular_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(47, *item));
                }
                pairs.push(CodePair::new_f64(48, obj.reflection_map_blend_factor));
                pairs.push(CodePair::new_i16(171, as_i16(obj.use_image_file_for_reflection_map)));
                pairs.push(CodePair::new_string(6, &obj.reflection_map_file_name));
                pairs.push(CodePair::new_i16(172, obj.reflection_map_projection_method as i16));
                pairs.push(CodePair::new_i16(173, obj.reflection_map_tiling_method as i16));
                pairs.push(CodePair::new_i16(174, obj.reflection_map_auto_transform_method as i16));
                for item in &obj.reflection_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(49, *item));
                }
                pairs.push(CodePair::new_f64(140, obj.opacity_factor));
                pairs.push(CodePair::new_f64(141, obj.opacity_map_blend_factor));
                pairs.push(CodePair::new_i16(175, as_i16(obj.use_image_file_for_opacity_map)));
                pairs.push(CodePair::new_string(7, &obj.opacity_map_file_name));
                pairs.push(CodePair::new_i16(176, obj.opacity_map_projection_method as i16));
                pairs.push(CodePair::new_i16(177, obj.opacity_map_tiling_method as i16));
                pairs.push(CodePair::new_i16(178, obj.opacity_map_auto_transform_method as i16));
                for item in &obj.opacity_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(142, *item));
                }
                pairs.push(CodePair::new_f64(143, obj.bump_map_blend_factor));
                pairs.push(CodePair::new_i16(179, as_i16(obj.use_image_file_for_bump_map)));
                pairs.push(CodePair::new_string(8, &obj.bump_map_file_name));
                pairs.push(CodePair::new_i16(270, obj.bump_map_projection_method as i16));
                pairs.push(CodePair::new_i16(271, obj.bump_map_tiling_method as i16));
                pairs.push(CodePair::new_i16(272, obj.bump_map_auto_transform_method as i16));
                for item in &obj.bump_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(144, *item));
                }
                pairs.push(CodePair::new_f64(145, obj.refraction_index));
                pairs.push(CodePair::new_f64(146, obj.refraction_map_blend_factor));
                pairs.push(CodePair::new_i16(273, as_i16(obj.use_image_file_for_refraction_map)));
                pairs.push(CodePair::new_string(9, &obj.refraction_map_file_name));
                pairs.push(CodePair::new_i16(274, obj.refraction_map_projection_method as i16));
                pairs.push(CodePair::new_i16(275, obj.refraction_map_tiling_method as i16));
                pairs.push(CodePair::new_i16(276, obj.refraction_map_auto_transform_method as i16));
                for item in &obj.refraction_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(147, *item));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(460, obj.color_bleed_scale));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(461, obj.indirect_dump_scale));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(462, obj.reflectance_scale));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(463, obj.transmittance_scale));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_bool(290, obj.is_two_sided));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(464, obj.luminance));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(270, obj.luminance_mode));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(271, obj.normal_map_method));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(465, obj.normal_map_strength));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(42, obj.normal_map_blend_factor));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(72, as_i16(obj.use_image_file_for_normal_map)));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_string(3, &obj.normal_map_file_name));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(73, obj.normal_map_projection_method as i16));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(74, obj.normal_map_tiling_method as i16));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(75, obj.normal_map_auto_transform_method as i16));
                }
                for item in &obj.normal_map_transformation_matrix.values() {
                    pairs.push(CodePair::new_f64(43, *item));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_bool(293, obj.is_anonymous));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(272, obj.global_illumination_mode));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(273, obj.final_gather_mode));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_string(300, &obj.gen_proc_name));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_bool(291, obj.gen_proc_boolean_value));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(271, obj.gen_proc_integer_value));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_f64(469, obj.gen_proc_real_value));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_string(301, &obj.gen_proc_text_value));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_bool(292, obj.gen_proc_table_end));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(62, obj.gen_proc_color_index_value.raw_value()));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i32(420, obj.gen_proc_color_rgb_value));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_string(430, &obj.gen_proc_color_name));
                }
                if version >= AcadVersion::R2007 {
                    pairs.push(CodePair::new_i16(270, obj.map_u_tile));
                }
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_f64(148, obj.translucence));
                }
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_i32(90, obj.self_illumination));
                }
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_f64(468, obj.reflectivity));
                }
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_i32(93, obj.illumination_model));
                }
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_i32(94, obj.channel_flags));
                }
            },
            ObjectType::MLeaderStyle(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbMLeaderStyle"));
                pairs.push(CodePair::new_i16(170, obj.content_type));
                pairs.push(CodePair::new_i16(171, obj.draw_m_leader_order_type));
                pairs.push(CodePair::new_i16(172, obj.draw_leader_order_type));
                pairs.push(CodePair::new_i32(90, obj.max_leader_segment_count));
                pairs.push(CodePair::new_f64(40, obj.first_segment_angle_constraint));
                pairs.push(CodePair::new_f64(41, obj.second_segment_angle_constraint));
                pairs.push(CodePair::new_i16(173, obj.leader_line_type));
                pairs.push(CodePair::new_i32(91, obj.leader_line_color));
                pairs.push(CodePair::new_string(340, &obj.__line_leader_type_handle.as_string()));
                pairs.push(CodePair::new_i32(92, obj.leader_line_weight));
                pairs.push(CodePair::new_bool(290, obj.enable_landing));
                pairs.push(CodePair::new_f64(42, obj.landing_gap));
                pairs.push(CodePair::new_bool(291, obj.enable_dogleg));
                pairs.push(CodePair::new_f64(43, obj.dogleg_length));
                pairs.push(CodePair::new_string(3, &obj.m_leader_style_description));
                pairs.push(CodePair::new_string(341, &obj.__arrowhead_handle.as_string()));
                pairs.push(CodePair::new_f64(44, obj.arrowhead_size));
                pairs.push(CodePair::new_string(300, &obj.default_m_text_contents));
                pairs.push(CodePair::new_string(342, &obj.__m_text_style_handle.as_string()));
                pairs.push(CodePair::new_i16(174, obj.text_left_attachment_type));
                pairs.push(CodePair::new_i16(175, obj.text_angle_type));
                pairs.push(CodePair::new_i16(176, obj.text_alignment_type));
                pairs.push(CodePair::new_i16(178, obj.text_right_attachment_type));
                pairs.push(CodePair::new_i32(93, obj.text_color));
                pairs.push(CodePair::new_f64(45, obj.text_height));
                pairs.push(CodePair::new_bool(292, obj.enable_frame_text));
                pairs.push(CodePair::new_bool(297, obj.always_align_text_left));
                pairs.push(CodePair::new_f64(46, obj.align_gap));
                pairs.push(CodePair::new_string(343, &obj.__block_content_handle.as_string()));
                pairs.push(CodePair::new_i32(94, obj.block_content_color));
                pairs.push(CodePair::new_f64(47, obj.block_content_x_scale));
                pairs.push(CodePair::new_f64(49, obj.block_content_y_scale));
                pairs.push(CodePair::new_f64(140, obj.block_content_z_scale));
                pairs.push(CodePair::new_bool(293, obj.enable_block_content_scale));
                pairs.push(CodePair::new_f64(141, obj.block_content_rotation));
                pairs.push(CodePair::new_bool(294, obj.enable_block_content_rotation));
                pairs.push(CodePair::new_i16(177, obj.block_content_connection_type));
                pairs.push(CodePair::new_f64(142, obj.scale));
                pairs.push(CodePair::new_bool(295, obj.overwrite_field_value));
                pairs.push(CodePair::new_bool(296, obj.is_annotative));
                pairs.push(CodePair::new_f64(143, obj.break_gap_size));
                pairs.push(CodePair::new_i16(271, obj.text_attachment_direction as i16));
                pairs.push(CodePair::new_i16(272, obj.bottom_text_attachment_direction as i16));
                pairs.push(CodePair::new_i16(273, obj.top_text_attachment_direction as i16));
            },
            ObjectType::MLineStyle(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbMlineStyle")));
                pairs.push(CodePair::new_string(2, &obj.style_name));
                pairs.push(CodePair::new_i16(70, obj.__flags as i16));
                pairs.push(CodePair::new_string(3, &obj.description));
                pairs.push(CodePair::new_i16(62, obj.fill_color.raw_value()));
                pairs.push(CodePair::new_f64(51, obj.start_angle));
                pairs.push(CodePair::new_f64(52, obj.end_angle));
                pairs.push(CodePair::new_i16(71, obj.elements.len() as i16));
                for item in &obj.elements {
                    pairs.push(CodePair::new_f64(49, item.offset));
                    pairs.push(CodePair::new_i16(62, item.color.raw_value()));
                    pairs.push(CodePair::new_string(6, &item.line_type));
                }
            },
            ObjectType::ObjectPointer(ref _obj) => {
            },
            ObjectType::PlotSettings(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbPlotSettings"));
                pairs.push(CodePair::new_string(1, &obj.page_setup_name));
                pairs.push(CodePair::new_string(2, &obj.printer_name));
                pairs.push(CodePair::new_string(4, &obj.paper_size));
                pairs.push(CodePair::new_string(6, &obj.plot_view_name));
                pairs.push(CodePair::new_f64(40, obj.unprintable_left_margin_size));
                pairs.push(CodePair::new_f64(41, obj.unprintable_bottom_margin_size));
                pairs.push(CodePair::new_f64(42, obj.unprintable_right_margin_size));
                pairs.push(CodePair::new_f64(43, obj.unprintable_top_margin_size));
                pairs.push(CodePair::new_f64(44, obj.paper_width));
                pairs.push(CodePair::new_f64(45, obj.paper_height));
                pairs.push(CodePair::new_f64(46, obj.plot_origin_x_value));
                pairs.push(CodePair::new_f64(47, obj.plot_origin_y_value));
                pairs.push(CodePair::new_f64(48, obj.plot_window_lower_left_x_value));
                pairs.push(CodePair::new_f64(49, obj.plot_window_lower_left_y_value));
                pairs.push(CodePair::new_f64(140, obj.plot_window_upper_right_x_value));
                pairs.push(CodePair::new_f64(141, obj.plot_window_upper_right_y_value));
                pairs.push(CodePair::new_f64(142, obj.custom_print_scale_numerator));
                pairs.push(CodePair::new_f64(143, obj.custom_print_scale_denominator));
                pairs.push(CodePair::new_i16(70, obj.plot_layout_flags as i16));
                pairs.push(CodePair::new_i16(72, obj.plot_paper_units as i16));
                pairs.push(CodePair::new_i16(73, obj.plot_rotation as i16));
                pairs.push(CodePair::new_i16(74, obj.plot_type as i16));
                pairs.push(CodePair::new_string(7, &obj.current_style_sheet));
                pairs.push(CodePair::new_i16(75, obj.standard_scale as i16));
                if version >= AcadVersion::R2004 {
                    pairs.push(CodePair::new_i16(76, obj.shade_plot_mode as i16));
                }
                if version >= AcadVersion::R2004 {
                    pairs.push(CodePair::new_i16(77, obj.shade_plot_resolution_level as i16));
                }
                if version >= AcadVersion::R2004 {
                    pairs.push(CodePair::new_i16(78, obj.shade_plot_custom_dpi as i16));
                }
                pairs.push(CodePair::new_f64(147, obj.standard_scale_value));
                pairs.push(CodePair::new_f64(148, obj.paper_image_origin_x));
                pairs.push(CodePair::new_f64(149, obj.paper_image_origin_y));
                pairs.push(CodePair::new_string(333, &obj.__shade_plot_object_handle.as_string()));
            },
            ObjectType::RapidRTRenderEnvironment(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbIBLBackground"));
                pairs.push(CodePair::new_i32(90, obj.subclass_version));
                pairs.push(CodePair::new_bool(290, obj.is_enabled));
                pairs.push(CodePair::new_string(1, &obj.image_based_lignting_map_file_name));
                pairs.push(CodePair::new_f64(40, obj.rotation_angle));
                pairs.push(CodePair::new_bool(290, obj.is_map_as_background));
                pairs.push(CodePair::new_string(340, &obj.__custom_background_handle.as_string()));
            },
            ObjectType::RapidRenderSettings(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbRenderSettings")));
                pairs.push(CodePair::new_i32(90, obj.subclass_version));
                pairs.push(CodePair::new_string(1, &obj.render_preset_name));
                pairs.push(CodePair::new_bool(290, obj.are_materials_enabled));
                pairs.push(CodePair::new_bool(290, obj.is_texture_sampling_enabled));
                pairs.push(CodePair::new_bool(290, obj.are_back_faces_enabled));
                pairs.push(CodePair::new_bool(290, obj.are_shadows_enabled));
                pairs.push(CodePair::new_string(1, &obj.preview_image_file_name));
                pairs.push(CodePair::new_string(1, &obj.render_preset_description));
                pairs.push(CodePair::new_i32(90, obj.display_index));
                pairs.push(CodePair::new_bool(290, obj.is_predefined_render_present));
                pairs.push(CodePair::new_string(100, &String::from("AcDbRapidRTRenderSettings")));
                pairs.push(CodePair::new_i32(90, obj.subclass_version));
                pairs.push(CodePair::new_i16(70, obj.render_duration as i16));
                pairs.push(CodePair::new_i32(90, obj.render_level_count_imit));
                pairs.push(CodePair::new_i32(90, obj.render_time_limit));
                pairs.push(CodePair::new_i16(70, obj.render_accuracy as i16));
                pairs.push(CodePair::new_i16(70, obj.sampling_filter as i16));
                pairs.push(CodePair::new_f64(40, obj.filter_width));
                pairs.push(CodePair::new_f64(40, obj.filter_height));
            },
            ObjectType::RasterVariables(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbRasterVariables"));
                pairs.push(CodePair::new_i32(90, obj.class_version));
                pairs.push(CodePair::new_i16(70, as_i16(obj.is_display_frame_image)));
                pairs.push(CodePair::new_i16(71, as_i16(obj.is_high_display_quality)));
                pairs.push(CodePair::new_i16(72, obj.image_units as i16));
            },
            ObjectType::MentalRayRenderSettings(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbRenderSettings")));
                pairs.push(CodePair::new_i32(90, obj.class_version_1));
                pairs.push(CodePair::new_string(1, &obj.preset_name));
                pairs.push(CodePair::new_bool(290, obj.render_materials));
                pairs.push(CodePair::new_i32(90, obj.texture_sampling_quality));
                pairs.push(CodePair::new_bool(290, obj.render_back_faces));
                pairs.push(CodePair::new_bool(290, obj.render_shadows));
                pairs.push(CodePair::new_string(1, &obj.preview_file_name));
                pairs.push(CodePair::new_string(100, &String::from("AcDbMentalRayRenderSettings")));
                pairs.push(CodePair::new_i32(90, obj.class_version_2));
                pairs.push(CodePair::new_i32(90, obj.minimum_sampling_rate));
                pairs.push(CodePair::new_i32(90, obj.maximum_sampling_rate));
                pairs.push(CodePair::new_i16(70, obj.sampling_filter_type as i16));
                pairs.push(CodePair::new_f64(40, obj.filter_width));
                pairs.push(CodePair::new_f64(40, obj.filter_height));
                pairs.push(CodePair::new_f64(40, obj.sampling_contrast_color_red));
                pairs.push(CodePair::new_f64(40, obj.sampling_contrast_color_green));
                pairs.push(CodePair::new_f64(40, obj.sampling_contrast_color_blue));
                pairs.push(CodePair::new_f64(40, obj.sampling_contrast_color_alpha));
                pairs.push(CodePair::new_i16(70, obj.shadow_mode as i16));
                pairs.push(CodePair::new_bool(290, obj.map_shadows));
                pairs.push(CodePair::new_bool(290, obj.ray_tracing));
                pairs.push(CodePair::new_i32(90, obj.ray_tracing_depth_reflections));
                pairs.push(CodePair::new_i32(90, obj.ray_tracing_depth_refractions));
                pairs.push(CodePair::new_i32(90, obj.ray_tracing_depth_maximum));
                pairs.push(CodePair::new_bool(290, obj.use_global_illumination));
                pairs.push(CodePair::new_i32(90, obj.sample_count));
                pairs.push(CodePair::new_bool(290, obj.use_global_illumination_radius));
                pairs.push(CodePair::new_f64(40, obj.global_illumination_radius));
                pairs.push(CodePair::new_i32(90, obj.photons_per_light));
                pairs.push(CodePair::new_i32(90, obj.global_illumination_depth_reflections));
                pairs.push(CodePair::new_i32(90, obj.global_illumination_depth_refractions));
                pairs.push(CodePair::new_i32(90, obj.global_illumination_depth_maximum));
                pairs.push(CodePair::new_bool(290, obj.use_final_gather));
                pairs.push(CodePair::new_i32(90, obj.final_gather_ray_count));
                pairs.push(CodePair::new_bool(290, obj.use_final_gather_minimum_radius));
                pairs.push(CodePair::new_bool(290, obj.use_final_gather_maximum_radius));
                pairs.push(CodePair::new_bool(290, obj.use_final_gather_pixels));
                pairs.push(CodePair::new_f64(40, obj.final_gather_sample_radius_minimum));
                pairs.push(CodePair::new_f64(40, obj.final_gather_sample_radius_maximum));
                pairs.push(CodePair::new_f64(40, obj.luminance_scale));
                pairs.push(CodePair::new_i16(70, obj.diagnostic_mode as i16));
                pairs.push(CodePair::new_i16(70, obj.diagnostic_grid_mode as i16));
                pairs.push(CodePair::new_f64(40, obj.grid_size));
                pairs.push(CodePair::new_i16(70, obj.diagnostic_photon_mode as i16));
                pairs.push(CodePair::new_i16(70, obj.diagnostic_bsp_mode as i16));
                pairs.push(CodePair::new_bool(290, obj.export_mi_statistics));
                pairs.push(CodePair::new_string(1, &obj.mi_statistics_file_name));
                pairs.push(CodePair::new_i32(90, obj.tile_size));
                pairs.push(CodePair::new_i16(70, obj.tile_order as i16));
                pairs.push(CodePair::new_i32(90, obj.memory_limit));
            },
            ObjectType::RenderEnvironment(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbRenderEnvironment"));
                pairs.push(CodePair::new_i32(90, obj.class_version));
                pairs.push(CodePair::new_bool(290, obj.is_fog_enabled));
                pairs.push(CodePair::new_bool(290, obj.is_background_fog_enabled));
                pairs.push(CodePair::new_i16(280, obj.fog_color_red as i16));
                pairs.push(CodePair::new_i16(280, obj.fog_color_green as i16));
                pairs.push(CodePair::new_i16(280, obj.fog_color_blue as i16));
                pairs.push(CodePair::new_f64(40, obj.near_fog_density_percent));
                pairs.push(CodePair::new_f64(40, obj.far_fog_density_percent));
                pairs.push(CodePair::new_f64(40, obj.near_clipping_plane_distance_percent));
                pairs.push(CodePair::new_f64(40, obj.far_clipping_plane_distance_percent));
                pairs.push(CodePair::new_bool(290, obj.use_environment_image));
                pairs.push(CodePair::new_string(1, &obj.environment_image_file_name));
            },
            ObjectType::RenderGlobal(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbRenderGlobal"));
                pairs.push(CodePair::new_i32(90, obj.class_version));
                pairs.push(CodePair::new_i32(90, obj.render_procedure as i32));
                pairs.push(CodePair::new_i32(90, obj.render_destination as i32));
                pairs.push(CodePair::new_bool(290, obj.save_to_file));
                pairs.push(CodePair::new_string(1, &obj.save_to_file_name));
                pairs.push(CodePair::new_i32(90, obj.image_width));
                pairs.push(CodePair::new_i32(90, obj.image_height));
                pairs.push(CodePair::new_bool(290, obj.use_predefined_presets_first));
                pairs.push(CodePair::new_bool(290, obj.use_high_info_level));
            },
            ObjectType::SectionManager(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbSectionManager")));
                pairs.push(CodePair::new_i16(70, as_i16(obj.requires_full_update)));
                pairs.push(CodePair::new_i32(90, obj.__section_entities_handle.len() as i32));
                for v in &obj.__section_entities_handle {
                    pairs.push(CodePair::new_string(330, &v.as_string()));
                }
            },
            ObjectType::SectionSettings(_) => { panic!("this case should have been covered in a custom writer"); },
            ObjectType::SortentsTable(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbSortentsTable"));
                for v in &obj.__entities_handle {
                    pairs.push(CodePair::new_string(331, &v.as_string()));
                }
                for v in &obj.__sort_items_handle {
                    pairs.push(CodePair::new_string(5, &v.as_string()));
                }
            },
            ObjectType::SpatialFilter(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbFilter")));
                pairs.push(CodePair::new_string(100, &String::from("AcDbSpatialFilter")));
                pairs.push(CodePair::new_i16(70, obj.clip_boundary_definition_points.len() as i16));
                for item in &obj.clip_boundary_definition_points {
                    pairs.push(CodePair::new_f64(10, item.x));
                    pairs.push(CodePair::new_f64(20, item.y));
                    pairs.push(CodePair::new_f64(30, item.z));
                }
                pairs.push(CodePair::new_f64(210, obj.clip_boundary_normal.x));
                pairs.push(CodePair::new_f64(220, obj.clip_boundary_normal.y));
                pairs.push(CodePair::new_f64(230, obj.clip_boundary_normal.z));
                pairs.push(CodePair::new_f64(11, obj.clip_boundary_origin.x));
                pairs.push(CodePair::new_f64(21, obj.clip_boundary_origin.y));
                pairs.push(CodePair::new_f64(31, obj.clip_boundary_origin.z));
                pairs.push(CodePair::new_i16(71, as_i16(obj.is_clip_boundary_enabled)));
                pairs.push(CodePair::new_i16(72, as_i16(obj.is_front_clipping_plane)));
                pairs.push(CodePair::new_f64(40, obj.front_clipping_plane_distance));
                pairs.push(CodePair::new_i16(73, as_i16(obj.is_back_clipping_plane)));
                pairs.push(CodePair::new_f64(41, obj.back_clipping_plane_distance));
                for item in &obj.inverse_transformation_matrix.values_row_major_4x3() {
                    pairs.push(CodePair::new_f64(40, *item));
                }
                for item in &obj.transformation_matrix.values_row_major_4x3() {
                    pairs.push(CodePair::new_f64(40, *item));
                }
            },
            ObjectType::SpatialIndex(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbIndex")));
                pairs.push(CodePair::new_f64(40, as_double_local(obj.timestamp)));
                pairs.push(CodePair::new_string(100, &String::from("AcDbSpatialIndex")));
            },
            ObjectType::SunStudy(_) => { panic!("this case should have been covered in a custom writer"); },
            ObjectType::TableStyle(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbTableStyle")));
                if version >= AcadVersion::R2010 {
                    pairs.push(CodePair::new_i16(280, obj.version as i16));
                }
                pairs.push(CodePair::new_string(3, &obj.description));
                pairs.push(CodePair::new_i16(70, obj.flow_direction as i16));
                pairs.push(CodePair::new_i16(71, obj.flags as i16));
                pairs.push(CodePair::new_f64(40, obj.horizontal_cell_margin));
                pairs.push(CodePair::new_f64(41, obj.vertical_cell_margin));
                pairs.push(CodePair::new_i16(280, as_i16(obj.is_title_suppressed)));
                pairs.push(CodePair::new_i16(281, as_i16(obj.is_column_heading_suppressed)));
                for item in &obj.cell_styles {
                    pairs.push(CodePair::new_string(7, &item.name));
                    pairs.push(CodePair::new_f64(140, item.text_height));
                    pairs.push(CodePair::new_i16(170, item.cell_alignment));
                    pairs.push(CodePair::new_i16(62, item.text_color.raw_value()));
                    pairs.push(CodePair::new_i16(63, item.cell_fill_color.raw_value()));
                    pairs.push(CodePair::new_i16(283, as_i16(item.is_background_color_enabled)));
                    pairs.push(CodePair::new_i32(90, item.cell_data_type));
                    pairs.push(CodePair::new_i32(91, item.cell_unit_type));
                    pairs.push(CodePair::new_i16(274, item.border_lineweight_1));
                    pairs.push(CodePair::new_i16(275, item.border_lineweight_2));
                    pairs.push(CodePair::new_i16(276, item.border_lineweight_3));
                    pairs.push(CodePair::new_i16(277, item.border_lineweight_4));
                    pairs.push(CodePair::new_i16(278, item.border_lineweight_5));
                    pairs.push(CodePair::new_i16(279, item.border_lineweight_6));
                    pairs.push(CodePair::new_i16(284, as_i16(item.is_border_1_visible)));
                    pairs.push(CodePair::new_i16(285, as_i16(item.is_border_2_visible)));
                    pairs.push(CodePair::new_i16(286, as_i16(item.is_border_3_visible)));
                    pairs.push(CodePair::new_i16(287, as_i16(item.is_border_4_visible)));
                    pairs.push(CodePair::new_i16(288, as_i16(item.is_border_5_visible)));
                    pairs.push(CodePair::new_i16(289, as_i16(item.is_border_6_visible)));
                    pairs.push(CodePair::new_i16(64, item.border_1_color.raw_value()));
                    pairs.push(CodePair::new_i16(65, item.border_2_color.raw_value()));
                    pairs.push(CodePair::new_i16(66, item.border_3_color.raw_value()));
                    pairs.push(CodePair::new_i16(67, item.border_4_color.raw_value()));
                    pairs.push(CodePair::new_i16(68, item.border_5_color.raw_value()));
                    pairs.push(CodePair::new_i16(69, item.border_6_color.raw_value()));
                }
            },
            ObjectType::UnderlayDefinition(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbUnderlayDefinition"));
                pairs.push(CodePair::new_string(1, &obj.file_name));
                pairs.push(CodePair::new_string(2, &obj.name));
            },
            ObjectType::VbaProject(ref obj) => {
                pairs.push(CodePair::new_string(100, &String::from("AcDbVbaProject")));
                pairs.push(CodePair::new_i32(90, obj.data.len() as i32));
                for item in &obj.__hex_data {
                    pairs.push(CodePair::new_binary(310, item.clone()));
                }
            },
            ObjectType::VisualStyle(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbVisualStyle"));
                pairs.push(CodePair::new_string(2, &obj.description));
                pairs.push(CodePair::new_i16(70, obj.type_code));
                pairs.push(CodePair::new_i16(71, obj.face_lighting_model as i16));
                pairs.push(CodePair::new_i16(72, obj.face_lighting_quality as i16));
                pairs.push(CodePair::new_i16(73, obj.face_color_mode as i16));
                pairs.push(CodePair::new_i32(90, obj.face_modifier as i32));
                pairs.push(CodePair::new_f64(40, obj.face_opacity_level));
                pairs.push(CodePair::new_f64(41, obj.face_specular_level));
                pairs.push(CodePair::new_i16(62, obj.color1.raw_value()));
                pairs.push(CodePair::new_i16(63, obj.color2.raw_value()));
                pairs.push(CodePair::new_i32(421, obj.face_style_mono_color));
                pairs.push(CodePair::new_i16(74, obj.edge_style_model as i16));
                pairs.push(CodePair::new_i32(91, obj.edge_style));
                pairs.push(CodePair::new_i16(64, obj.edge_intersection_color.raw_value()));
                pairs.push(CodePair::new_i16(65, obj.edge_obscured_color.raw_value()));
                pairs.push(CodePair::new_i16(75, obj.edge_obscured_line_type));
                pairs.push(CodePair::new_i16(175, obj.edge_intersection_line_type));
                pairs.push(CodePair::new_f64(42, obj.edge_crease_angle));
                pairs.push(CodePair::new_i32(92, obj.edge_modifiers));
                pairs.push(CodePair::new_i16(66, obj.edge_color.raw_value()));
                pairs.push(CodePair::new_f64(43, obj.edge_opacity_level));
                pairs.push(CodePair::new_i16(76, obj.edge_width));
                pairs.push(CodePair::new_i16(77, obj.edge_overhang));
                pairs.push(CodePair::new_i16(78, obj.edge_jitter));
                pairs.push(CodePair::new_i16(67, obj.edge_silhouette_color.raw_value()));
                pairs.push(CodePair::new_i16(79, obj.edge_silhouette_width));
                pairs.push(CodePair::new_i16(170, obj.edge_halo_gap));
                pairs.push(CodePair::new_i16(171, obj.edge_iso_line_count as i16));
                pairs.push(CodePair::new_bool(290, obj.hide_edge_line_precision));
                pairs.push(CodePair::new_i16(174, obj.edge_style_apply_flags as i16));
                pairs.push(CodePair::new_i32(93, obj.display_style_settings));
                pairs.push(CodePair::new_f64(44, obj.brightness));
                pairs.push(CodePair::new_i16(173, obj.shadow_type));
                pairs.push(CodePair::new_bool(291, obj.internal_flag));
            },
            ObjectType::WipeoutVariables(ref obj) => {
                pairs.push(CodePair::new_str(100, "AcDbRasterVariables"));
                pairs.push(CodePair::new_i32(90, obj.class_version));
                pairs.push(CodePair::new_i16(70, as_i16(obj.display_image_frame)));
            },
            ObjectType::XRecordObject(_) => { panic!("this case should have been covered in a custom writer"); },
        }
    }
}
