mod online_accounts;
use online_accounts::OAuth2BasedProxyBlocking;

use clap::{Arg, Command};
use zbus::blocking::{fdo::ObjectManagerProxy, Connection};

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
    let app = Command::new("goa-oauth2")
        .version("1.1")
        .about("asks GNOME Online Accounts for OAuth2 tokens")
        .author("Hank Donnay <hdonnay@gmail.com>")
        .arg(Arg::new("ACCOUNT").required(true));

    let conn = Connection::session()?;
    let om = ObjectManagerProxy::builder(&conn)
        .destination(GOA_DEST)?
        .path(GOA_PATH)?
        .build()?;
    let m = app.get_matches();
    let want = m
        .get_one::<String>("ACCOUNT")
        .expect("account name missing");
    let dest = om.inner().destination().to_owned();

    let tok = om
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
        .map(|path| {
            let (tok, _expiry) = OAuth2BasedProxyBlocking::builder(&conn)
                .destination(&dest)?
                .path(path)?
                .build()?
                .get_access_token()?;
            Ok(tok)
        })
        .find_map(|t: Result<String, Box<dyn std::error::Error>>| t.ok())
        .ok_or_else(|| Box::new(NotFound))?;

    println!("{}", tok);
    Ok(())
}
