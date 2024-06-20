use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use url::Url;
use worldtree_compiler::{Conditional, Model, Text, TextNode};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ArtistInfo {
    name: &'static str,
    url: Option<&'static str>,
    license: CC,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
enum CC {
    BY,
    ZERO,
}

impl ArtistInfo {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            url: None,
            license: CC::BY,
        }
    }

    fn with_url(self, url: &'static str) -> Self {
        Self {
            name: self.name,
            url: Some(url),
            license: self.license,
        }
    }

    fn with_license(self, license: CC) -> Self {
        Self {
            name: self.name,
            url: self.url,
            license,
        }
    }
}

lazy_static! {
    static ref ARTISTS: HashMap<&'static str, ArtistInfo> = {
        let mut artists = HashMap::new();

        artists.insert("lorc", ArtistInfo::new("Lorc").with_url("https://lorcblog.blogspot.com"));
        artists.insert("delapouite", ArtistInfo::new("Delapouite").with_url("https://delapouite.com"));
        artists.insert("john-colburn", ArtistInfo::new("John Colburn"));
        artists.insert("felbrigg", ArtistInfo::new("Felbrigg").with_url("https://blackdogofdoom.blogspot.co.uk"));
        artists.insert("john-redman", ArtistInfo::new("John Redman"));
        artists.insert("carl-olsen", ArtistInfo::new("Carl Olsen").with_url("https://twitter.com/unstoppableCarl"));
        artists.insert("sbed", ArtistInfo::new("Sbed").with_url("https://opengameart.org/content/95-game-icons"));
        artists.insert("priorblue", ArtistInfo::new("PriorBlue"));
        artists.insert("willdabeast", ArtistInfo::new("Wildabeast").with_url("https://wjbstories.blogspot.com"));
        artists.insert("viscious-speed", ArtistInfo::new("Viscious Speed").with_url("https://viscious-speed.deviantart.com").with_license(CC::ZERO));
        artists.insert("lord-berandas", ArtistInfo::new("Lord Berandas").with_url("https://berandas.deviantart.com"));
        artists.insert("irongamer", ArtistInfo::new("Irongamer").with_url("https://ecesisllc.wixsite.com/home"));
        artists.insert("heavenlydog", ArtistInfo::new("HeavenlyDog").with_url("https://www.gnomosygoblins.blogspot.com"));
        artists.insert("lucas", ArtistInfo::new("Lucas"));
        artists.insert("faithtoken", ArtistInfo::new("Faithtoken").with_url("https://fungustoken.deviantart.com"));
        artists.insert("skoll", ArtistInfo::new("Skoll"));
        artists.insert("andy-meneely", ArtistInfo::new("Andy Meneely").with_url("https://www.se.rit.edu/~andy"));
        artists.insert("cathelineau", ArtistInfo::new("Cathelineau"));
        artists.insert("kier-heyl", ArtistInfo::new("Kier Heyl"));
        artists.insert("aussiesim", ArtistInfo::new("Aussiesim"));
        artists.insert("sparker", ArtistInfo::new("Sparker"));
        artists.insert("zeromancer", ArtistInfo::new("Zeromancer").with_license(CC::ZERO));
        artists.insert("rihlsul", ArtistInfo::new("Rihlsul"));
        artists.insert("quoting", ArtistInfo::new("Quoting"));
        artists.insert("guard13007", ArtistInfo::new("Guard13007"));
        artists.insert("darkzaitzev", ArtistInfo::new("DarkZaitzev").with_url("https://darkzaitzev.deviantart.com"));
        artists.insert("spencerdub", ArtistInfo::new("SpencerDub"));
        artists.insert("generalace135", ArtistInfo::new("GeneralAce135"));
        artists.insert("zajkonur", ArtistInfo::new("Zajkonur"));
        artists.insert("catsu", ArtistInfo::new("Catsu"));
        artists.insert("Starseeker", ArtistInfo::new("Starseeker"));
        artists.insert("pepijn-poolman", ArtistInfo::new("Pepijn Poolman"));
        artists.insert("pierre-leducq", ArtistInfo::new("Pierre Leducq"));
        artists.insert("caro-asercion", ArtistInfo::new("Caro Asercion"));
        artists.insert("various-artists", ArtistInfo::new("various artists"));

        artists
    };
}

fn gather_icons(icon: &Conditional<String>) -> Vec<String> {
    match icon {
        Conditional::Always(icon) => vec!(icon.clone()),
        Conditional::Conditionally(_, value, next) => {
            let mut result = vec!(value.clone());
            result.extend(gather_icons(&next));
            result
        }
    }
}

fn gather_game_icons(model: &Model) -> Vec<String> {
    let base_url = Url::parse("https://www.example.com/").unwrap();
    let url_options = Url::options().base_url(Some(&base_url));
    let mut game_icons_set = HashSet::new();

    let mut urls = Vec::new();

    for storylet in &model.storylets {
        if let Some(icon) = &storylet.icon {
            urls.extend(gather_icons(icon));
        }

        if let Some(choices) = &storylet.choices {
            for group in &choices.groups {
                for choice in &group.choices {
                    if let Some(icon) = &choice.icon {
                        urls.extend(gather_icons(icon));
                    }
                }
            }
        }
    }

    for url in urls {
        if let Ok(url) = url_options.parse(&url) {
            if url.scheme() == "game-icons" {
                game_icons_set.insert(url.path().to_owned());
            }
        }
    }

    game_icons_set.iter().map(|icon| String::from(icon)).collect()
}

pub fn add_game_icons_credits(model: &mut Model) {
    let icons = gather_game_icons(model);

    let mut artist_keys = HashSet::new();
    let mut artist_infos = Vec::new();
    for icon in &icons {
        if let Some(artist_key) = icon.split("/").next() {
            if artist_keys.insert(artist_key) {
                if let Some(info) = ARTISTS.get(artist_key) {
                    artist_infos.push(info);
                }
            }
        }
    }

    artist_infos.sort();

    let mut ccby_artist_list: Text = Vec::new();
    let mut cczero_artist_list: Text = Vec::new();
    for artist in artist_infos {
        if let CC::BY = artist.license {
            if !ccby_artist_list.is_empty() {
                ccby_artist_list.push(TextNode::Plain(", ".to_string()));
            }
            if let Some(url) = artist.url {
                ccby_artist_list.push(TextNode::Anchor(url.to_string(), vec!(TextNode::Plain(artist.name.to_string()))));
            } else {
                ccby_artist_list.push(TextNode::Plain(artist.name.to_string()));
            }
        } else if let CC::ZERO = artist.license {
            if !cczero_artist_list.is_empty() {
                cczero_artist_list.push(TextNode::Plain(", ".to_string()));
            }
            if let Some(url) = artist.url {
                cczero_artist_list.push(TextNode::Anchor(url.to_string(), vec!(TextNode::Plain(artist.name.to_string()))));
            } else {
                cczero_artist_list.push(TextNode::Plain(artist.name.to_string()));
            }
        }
    }

    if !ccby_artist_list.is_empty() {
        ccby_artist_list.insert(0, TextNode::Plain("Icons by ".to_string()));
        ccby_artist_list.push(TextNode::Plain(" licensed under the ".to_string()));
        ccby_artist_list.push(TextNode::Anchor("https://creativecommons.org/licenses/by/3.0/".to_string(), vec!(TextNode::Plain("CC BY 3.0 license".to_string()))));
        model.meta.credits.push(ccby_artist_list);
    }

    if !cczero_artist_list.is_empty() {
        cczero_artist_list.insert(0, TextNode::Plain("Icons by ".to_string()));
        cczero_artist_list.push(TextNode::Plain(" licensed under the ".to_string()));
        cczero_artist_list.push(TextNode::Anchor("https://creativecommons.org/publicdomain/zero/1.0/".to_string(), vec!(TextNode::Plain("CC0 license".to_string()))));
        model.meta.credits.push(cczero_artist_list);
    }
}
