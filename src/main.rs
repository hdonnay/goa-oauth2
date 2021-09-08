mod online_accounts;
use online_accounts::OAuth2BasedProxy;

use clap::{App, Arg};
use zbus::fdo::ObjectManagerProxy;
use zvariant::OwnedObjectPath;

#[derive(Debug)]
struct NotFound;
impl std::error::Error for NotFound {}
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "account not found")
    }
}

static GOA_PATH: &str = "/org/gnome/OnlineAccounts";
static GOA_DEST: &str = "org.gnome.OnlineAccounts";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("goa-oauth2")
        .version("1.1")
        .about("asks GNOME Online Accounts for OAuth2 tokens")
        .author("Hank Donnay <hdonnay@gmail.com>")
        .arg(Arg::with_name("ACCOUNT").required(true));

    let conn = zbus::Connection::session()?;
    let om = ObjectManagerProxy::builder(&conn)
        .destination(GOA_DEST)?
        .path(GOA_PATH)?
        .build()?;
    let m = app.get_matches();
    let want = m.value_of("ACCOUNT").expect("account name missing");
    let dest = om.inner().destination().to_owned();

    for path in om
        .get_managed_objects()?
        .iter()
        .filter(|(_, i)| {
            i.get("org.gnome.OnlineAccounts.Account").is_some()
                && i.get("org.gnome.OnlineAccounts.OAuth2Based").is_some()
        })
        .filter_map(|(path, i)| {
            // These unwraps should be fine do to, because of the filter.
            if i.get("org.gnome.OnlineAccounts.Account")
                .unwrap()
                .get("Identity")
                .unwrap()
                .downcast_ref::<str>()
                .unwrap()
                != want
            {
                None
            } else {
                Some(path)
            }
        })
        .collect::<Vec<&OwnedObjectPath>>()
    {
        let (tok, _expiry) = OAuth2BasedProxy::builder(&conn)
            .destination(dest)?
            .path(path)?
            .build()?
            .get_access_token()?;
        println!("{}", tok);
        return Ok(());
    }

    Err(Box::new(NotFound))
}
