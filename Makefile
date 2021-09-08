
all: src/online_accounts.rs

clean:
	rm dbus-interfaces.xml

dbus-interfaces.xml:
	curl -sSLf https://gitlab.gnome.org/GNOME/gnome-online-accounts/-/raw/master/data/dbus-interfaces.xml?inline=false\
		>$@

src/online_accounts.rs: dbus-interfaces.xml
	zbus-xmlgen $< >$@
