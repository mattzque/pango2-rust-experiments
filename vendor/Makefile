PREFIX=/opt/pango2
export PKG_CONFIG_PATH := /opt/pango2/lib/x86_64-linux-gnu/pkgconfig/:${PKG_CONFIG_PATH}
export LD_LIBRARY_PATH := /opt/pango2/lib/x86_64-linux-gnu/:${LD_LIBRARY_PATH}

install-cairo:
	cd cairo/ && ( \
		mkdir -p build && \
		meson setup --prefix=${PREFIX} --buildtype release build && \
		meson compile -C build/ && \
		meson install -C build/)

install-harfbuzz:
	cd harfbuzz/ && ( \
		mkdir -p build && \
		meson setup --prefix=${PREFIX} --buildtype release build && \
		meson compile -C build/ && \
		meson install -C build/)

install-pango: install-cairo install-harfbuzz
	cd pango/ && ( \
		(patch -p1 -f -i ../pango-683.diff || true) && \
		mkdir -p build && \
		meson setup --prefix=${PREFIX} --buildtype release build && \
		meson compile -C build/ && \
		meson install -C build/)

clean:
	rm cairo/build -R
	rm pango/build -R
	rm harfbuzz/build -R

install: install-pango