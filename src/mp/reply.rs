use exile::{Document, Element};
use serde_derive::{Deserialize, Serialize};

use crate::SdkResult;

#[derive(Debug, Serialize, Deserialize)]
pub enum Reply {
    Text(Text),
    Image(Image),
    Voice(Voice),
    Video(Video),
    Music(Music),
    News(News),
    Unknow,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub media_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    pub media_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub media_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Music {
    pub thumb_media_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub music_url: Option<String>,
    pub hq_music_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    pub article_count: i32,
    pub articles: Vec<Articles>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Articles {
    pub item: ArticleItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleItem {
    #[serde(alias = "Title")]
    pub title: String,

    #[serde(alias = "Description")]
    pub description: String,

    #[serde(alias = "PicUrl")]
    pub pic_url: String,

    #[serde(alias = "Url")]
    pub url: String,
}

pub fn reply_to_xml<S: Into<String>>(reply: Reply, from: S, to: S) -> SdkResult<String> {
    let mut root = Element::from_name("xml");

    let mut to_user_name = Element::from_name("ToUserName");
    to_user_name
        .add_cdata(to)
        .map_err(exile::error::Error::from)?;
    root.add_child(to_user_name);

    let mut from_uesr_name = Element::from_name("FromUserName");
    from_uesr_name
        .add_cdata(from)
        .map_err(exile::error::Error::from)?;
    root.add_child(from_uesr_name);

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = timestamp.as_secs();
    let mut create_time = Element::from_name("CreateTime");
    create_time.add_text(secs.to_string());
    root.add_child(create_time);

    let mut msg_type = Element::from_name("MsgType");
    let body = match reply {
        Reply::Text(t) => {
            msg_type
                .add_cdata("text")
                .map_err(exile::error::Error::from)?;
            root.add_child(msg_type);
            let mut content = Element::from_name("Content");
            content
                .add_cdata(t.content)
                .map_err(exile::error::Error::from)?;
            content
        }
        Reply::Image(i) => {
            msg_type
                .add_cdata("image")
                .map_err(exile::error::Error::from)?;
            root.add_child(msg_type);
            let mut image = Element::from_name("Iamge");
            let mut media_id = Element::from_name("MediaId");
            media_id
                .add_cdata(i.media_id)
                .map_err(exile::error::Error::from)?;
            image.add_child(media_id);
            image
        }
        Reply::Voice(v) => {
            msg_type
                .add_cdata("voice")
                .map_err(exile::error::Error::from)?;
            root.add_child(msg_type);
            let mut voice = Element::from_name("Voice");
            let mut media_id = Element::from_name("MediaId");
            media_id
                .add_cdata(v.media_id)
                .map_err(exile::error::Error::from)?;
            voice.add_child(media_id);
            voice
        }
        Reply::Video(v) => {
            msg_type
                .add_cdata("video")
                .map_err(exile::error::Error::from)?;
            root.add_child(msg_type);
            let mut video = Element::from_name("Video");
            let mut media_id = Element::from_name("MediaId");
            media_id
                .add_cdata(v.media_id)
                .map_err(exile::error::Error::from)?;
            video.add_child(media_id);
            if let Some(t) = v.title {
                let mut title = Element::from_name("Title");
                title.add_cdata(t).map_err(exile::error::Error::from)?;
                video.add_child(title);
            }
            if let Some(t) = v.description {
                let mut des = Element::from_name("Description");
                des.add_cdata(t).map_err(exile::error::Error::from)?;
                video.add_child(des);
            }
            video
        }
        Reply::Music(m) => {
            msg_type
                .add_cdata("music")
                .map_err(exile::error::Error::from)?;
            root.add_child(msg_type);
            let mut music = Element::from_name("Music");
            let mut thumb_media_id = Element::from_name("ThumbMediaId");
            thumb_media_id
                .add_cdata(m.thumb_media_id)
                .map_err(exile::error::Error::from)?;
            if let Some(t) = m.title {
                let mut title = Element::from_name("Title");
                title.add_cdata(t).map_err(exile::error::Error::from)?;
                music.add_child(title);
            }
            if let Some(t) = m.description {
                let mut des = Element::from_name("Description");
                des.add_cdata(t).map_err(exile::error::Error::from)?;
                music.add_child(des);
            }
            if let Some(t) = m.music_url {
                let mut music_url = Element::from_name("MusicUrl");
                music_url.add_cdata(t).map_err(exile::error::Error::from)?;
                music.add_child(music_url);
            }
            if let Some(t) = m.hq_music_url {
                let mut hq = Element::from_name("HQMusicUrl");
                hq.add_cdata(t).map_err(exile::error::Error::from)?;
                music.add_child(hq);
            }
            music.add_child(thumb_media_id);
            music
        }
        Reply::News(n) => {
            msg_type
                .add_cdata("news")
                .map_err(exile::error::Error::from)?;
            root.add_child(msg_type);

            let mut article_count = Element::from_name("ArticleCount");
            article_count.add_text(n.article_count.to_string());
            root.add_child(article_count);

            let mut articles = Element::from_name("Articles");
            for i in n.articles {
                let mut item = Element::from_name("item");
                let mut title = Element::from_name("Title");
                title
                    .add_cdata(i.item.title)
                    .map_err(exile::error::Error::from)?;
                item.add_child(title);
                let mut des = Element::from_name("Description");
                des.add_cdata(i.item.description)
                    .map_err(exile::error::Error::from)?;
                item.add_child(des);
                let mut pic_url = Element::from_name("PicUrl");
                pic_url
                    .add_cdata(i.item.pic_url)
                    .map_err(exile::error::Error::from)?;
                item.add_child(pic_url);
                let mut url = Element::from_name("Url");
                url.add_cdata(i.item.url)
                    .map_err(exile::error::Error::from)?;
                item.add_child(url);
                articles.add_child(item);
            }
            articles
        }
        Reply::Unknow => unreachable!(),
    };
    root.add_child(body);

    let doc = Document::from_root(root);
    Ok(doc.to_string())
}
