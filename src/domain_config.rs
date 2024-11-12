#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
	#[serde(rename = "id")]
	pub id: Option<i32>,

	#[serde(rename = "ip")]
	pub ip: Option<String>,

	#[serde(rename = "name")]
	pub name: Option<String>,

	#[serde(rename = "status")]
	pub status: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CloakConfig {
	#[serde(rename = "id")]
	pub id: Option<String>,

	#[serde(rename = "name")]
	pub name: Option<String>,

	#[serde(rename = "allowRegion")]
	pub allow_region: Option<String>,

	#[serde(rename = "useCloakProvider")]
	pub use_cloak_provider: Option<bool>,

	#[serde(rename = "previewSecret")]
	pub preview_secret: Option<String>,

	#[serde(rename = "enableRegionDetection")]
	pub enable_region_detection: Option<bool>,

	#[serde(rename = "enableSpiderDetection")]
	pub enable_spider_detection: Option<bool>,

	#[serde(rename = "enableLanguageDetection")]
	pub enable_language_detection: Option<bool>,

	#[serde(rename = "enableProxyDetection")]
	pub enable_proxy_detection: Option<bool>,

	#[serde(rename = "enableUaDetection")]
	pub enable_ua_detection: Option<bool>,

	#[serde(rename = "enableBlacklistIpDetection")]
	pub enable_blacklist_ip_detection: Option<bool>,

	#[serde(rename = "enableBlacklistIpCollection")]
	pub enable_blacklist_ip_collection: Option<bool>,

	#[serde(rename = "hidden")]
	pub hidden: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Landing {
	#[serde(rename = "id")]
	pub id: Option<i32>,

	#[serde(rename = "createdBy")]
	pub created_by: Option<i32>,

	#[serde(rename = "createdDate")]
	pub created_date: Option<i64>,

	#[serde(rename = "lastModifiedBy")]
	pub last_modified_by: Option<i32>,

	#[serde(rename = "lastModifiedDate")]
	pub last_modified_date: Option<i64>,

	#[serde(rename = "deptId")]
	pub dept_id: Option<String>,

	#[serde(rename = "uuid")]
	pub uuid: Option<String>,

	#[serde(rename = "name")]
	pub name: Option<String>,

	#[serde(rename = "version")]
	pub version: Option<i32>,

	#[serde(rename = "isPublic")]
	pub is_public: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Orders {
	#[serde(rename = "id")]
	pub id: Option<i32>,

	#[serde(rename = "createdBy")]
	pub created_by: Option<i32>,

	#[serde(rename = "createdDate")]
	pub created_date: Option<i64>,

	#[serde(rename = "lastModifiedBy")]
	pub last_modified_by: Option<i32>,

	#[serde(rename = "lastModifiedDate")]
	pub last_modified_date: Option<i64>,

	#[serde(rename = "deptId")]
	pub dept_id: Option<String>,

	#[serde(rename = "businessId")]
	pub business_id: Option<String>,

	#[serde(rename = "businessName")]
	pub business_name: Option<String>,

	#[serde(rename = "operatorNickname")]
	pub operator_nickname: Option<String>,

	#[serde(rename = "link")]
	pub link: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Websites {
	#[serde(rename = "id")]
	pub id: Option<i32>,

	#[serde(rename = "createdBy")]
	pub created_by: Option<i32>,

	#[serde(rename = "createdDate")]
	pub created_date: Option<i64>,

	#[serde(rename = "lastModifiedBy")]
	pub last_modified_by: Option<i32>,

	#[serde(rename = "lastModifiedDate")]
	pub last_modified_date: Option<i64>,

	#[serde(rename = "deptId")]
	pub dept_id: Option<String>,

	#[serde(rename = "path")]
	pub path: Option<String>,

	#[serde(rename = "type")]
	pub websites_type: Option<String>,

	#[serde(rename = "cloakConfig")]
	pub cloak_config: Option<CloakConfig>,

	#[serde(rename = "landing")]
	pub landing: Option<Landing>,

	#[serde(rename = "orders")]
	pub orders: Option<Vec<Orders>>,

	#[serde(rename = "targetLink")]
	pub target_link: Option<String>,

	#[serde(rename = "extraScript")]
	pub extra_script: Option<String>,

	#[serde(rename = "pixelId")]
	pub pixel_id: Option<String>,

	#[serde(rename = "banRedirectLink")]
	pub ban_redirect_link: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainConfig {
	#[serde(rename = "id")]
	pub id: Option<i32>,

	#[serde(rename = "domain")]
	pub domain: Option<String>,

	#[serde(rename = "server")]
	pub server: Option<Server>,

	#[serde(rename="ssl")]
	pub ssl: Option<bool>,
	
	#[serde(rename = "websites")]
	pub websites: Option<Vec<Websites>>,
}
use serde::{Serialize, Deserialize};
