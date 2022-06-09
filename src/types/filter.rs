pub struct DateFilters;
pub struct TypeFilters;
pub struct DurationFilters;
pub struct FeatureFilters;
pub struct SortFilters;
impl DateFilters {
    pub const HOUR: &'static str  = "EgIIAQ%3D%3D";
    pub const TODAY: &'static str = "EgIIAg%3D%3D";
    pub const WEEK: &'static str  = "EgIIAw%3D%3D";
    pub const MONTH: &'static str = "EgIIBA%3D%3D";
    pub const YEAR: &'static str  = "EgIIBQ%3D%3D";
}
  
impl TypeFilters {
    pub const VIDEO: &'static str    = "EgIQAQ%3D%3D";
    pub const CHANNEL: &'static str  = "EgIQAg%3D%3D";
    pub const PLAYLIST: &'static str = "EgIQAw%3D%3D";
    pub const MOVIE: &'static str    = "EgIQBA%3D%3D";
  }
  
impl DurationFilters {
    pub const SHORT: &'static str  = "EgIYAQ%3D%3D";
    pub const MEDIUM: &'static str = "EgIYAw%3D%3D";
    pub const LONG: &'static str   = "EgIYAg%3D%3D";
  }
  
impl FeatureFilters {
    pub const LIVE: &'static str= "EgJAAQ%3D%3D";
    pub const FOUR_K: &'static str= "EgJwAQ%3D%3D";
    pub const HD: &'static str= "EgIgAQ%3D%3D";
    pub const SUBTITLES: &'static str  = "EgIoAQ%3D%3D";
    pub const CCOMMONS: &'static str   = "EgIwAQ%3D%3D";
    pub const THREE_SIXTY: &'static str = "EgJ4AQ%3D%3D";
    pub const VR180: &'static str      = "EgPQAQE%3D";
    pub const THREE_D: &'static str     = "EgI4AQ%3D%3D";
    pub const HDR: &'static str        = "EgPIAQE%3D";
    pub const LOCATION: &'static str   = "EgO4AQE%3D";
    pub const PURCHASED: &'static str  = "EgJIAQ%3D%3D";
  }
  
impl  SortFilters {
    pub const RELEVANCE: &'static str = "";
    pub const DATE: &'static str      = "CAI%3D";
    pub const VIEWS: &'static str     = "CAM%3D";
    pub const RATING: &'static str    = "CAE%3D";
  }