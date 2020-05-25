
// https://video-api.wsj.com/api-video/find_all_videos.asp
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct WSJ {
//    pub items: Vec<WSJVideos>,
//}
//
//impl WSJ {
//    pub fn to_records(&self) -> Vec<Vec<String>> {
//        let mut recs: Vec<Vec<String>> = Vec::new();
//        for hl in self.items.iter() {
//            recs.push(WSJVideos::to_record(hl));
//        }
//        return recs;
//    }
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct WSJVideos {
//    pub id: String,
//    pub unix_creation_date: i64,
//    pub name: String,
//    pub description: String,
//    pub duration: String,
//    #[serde(rename = "thumbnailURL")]
//    pub thumbnail_url: Option<String>,
//    #[serde(rename = "videoURL")]
//    pub video_url: Option<String>,
//    #[serde(rename = "emailURL")]
//    pub email_url: Option<String>,
//    #[serde(rename = "doctypeID")]
//    pub doctype_id: Option<String>,
//    pub column: Option<String>,
//}

//impl WSJVideos {
//    pub fn to_record(&self) -> Vec<String> {
//        let rec: Vec<String> = vec![
//            self.id.to_string(),
//            self.unix_creation_date.to_string(),
//            self.name.replace(",", ";").to_string(),
//            self.description.replace(",", ";").to_string(),
//            self.duration.to_string(),
//            self.column.clone(),
//            self.doctype_id.clone(),
//            self.email_url.clone(),
//            self.thumbnail_url.clone(),
//        ];
//        return rec;
//    }
//}
