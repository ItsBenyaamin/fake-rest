
pub enum ContentType {
    // documents
    TXT,
    PDF,

    // images
    PNG,
    JPG,
    JPEG,

    // videos
    MP4,
    MKV,
    MPEG,
    FLV,
    M4V,

    // audio
    MP3,
    M4A,
    OGG,
    AAC,
    
    // compression
    ZIP,
    RAR,
    TAR,
    GZIP,
    Z7,

    OTHER,
}

impl ContentType {

    pub fn get_mime_type(extension: &str) -> String {
        let content_type: ContentType = extension.into();
        content_type.into()
    }

}

impl From<&str> for ContentType {
    fn from(s: &str) -> Self {
        match s {
            "txt"  => ContentType::TXT,
            "pdf"  => ContentType::PDF ,
            "png"  => ContentType::PNG,
            "jpg"  => ContentType::JPG,
            "jpeg" => ContentType::JPEG,
            "mp4"  => ContentType::MP4,
            "mkv"  => ContentType::MKV,
            "mpeg" => ContentType::MPEG,
            "flv"  => ContentType::FLV,
            "m4v"  => ContentType::M4V,
            "mp3"  => ContentType::MP3,
            "m4a"  => ContentType::M4A,
            "ogg"  => ContentType::OGG,
            "aac"  => ContentType::AAC,
            "zip"  => ContentType::ZIP,
            "rar"  => ContentType::RAR,
            "tar"  => ContentType::TAR,
            "gzip" => ContentType::GZIP,
            "7z"   => ContentType::Z7,
            _      => ContentType::OTHER,
        }
    }
}

impl From<ContentType> for String {
    fn from(content_type : ContentType) -> Self {
        match content_type {
            ContentType::TXT =>  "text/plain".to_string(),
            ContentType::PDF =>  "application/pdf".to_string(),
            ContentType::PNG =>  "image/png".to_string(),
            ContentType::JPG | ContentType::JPEG 
                =>  "image/jpeg".to_string(),
            ContentType::MP4 =>  "video/mp4".to_string(),
            ContentType::MKV =>  "video/x-matroska".to_string(),
            ContentType::MPEG => "video/mpeg".to_string(),
            ContentType::FLV =>  "video/x-flv".to_string(),
            ContentType::M4V =>  "video/x-m4v".to_string(),
            ContentType::MP3 =>  "audio/mpeg".to_string(),
            ContentType::M4A =>  "audio/m4a".to_string(),
            ContentType::OGG =>  "audio/ogg".to_string(),
            ContentType::AAC =>  "audio/aac".to_string(),
            ContentType::ZIP =>  "application/zip".to_string(),
            ContentType::RAR =>  "application/vnd.rar".to_string(),
            ContentType::TAR =>  "application/x-tar".to_string(),
            ContentType::GZIP => "application/gzip".to_string(),
            ContentType::Z7 =>   "application/x-7z-compressed".to_string(),
            ContentType::OTHER => "application/octet-stream".to_string(),
        }
    }
}