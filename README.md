# Experiments with Rust and Pango 2

Pango 2 has not yet seen any official release, but I saw pango 2 referred to a few times in various places so I wanted to give it a try. So consider this more experimental.

One of the very common issues that a lot of people run into (me included) when trying to integrate Pango 1 into things is custom font loading. That is the ability to load font faces from disk or even memory. 

There is an [old article](http://mces.blogspot.com/2015/05/how-to-use-custom-application-fonts.html) from 2015 on the subject by Behdad Esfahbod that talks about ways to accomplish this, involving the platform-dependent font enumeration systems. For instance, [node-canvas](https://github.com/Automattic/node-canvas) is a good example for how to accomplish this with Pango 1, although it seems to have caused them [all kinds of issues](https://github.com/Automattic/node-canvas/issues?q=custom+font).

Pango 2 makes all of this much simpler, [from the documentation](https://gitlab.gnome.org/GNOME/pango/-/blob/pango2/docs/pango_fonts.md):

> The default font map used by Pango will contain the fonts that are available
> via the font enumeration APIs of the system (for Linux, that is fontconfig).
> For special situations (such as writing Pango tests), it can appropriate
> to create an empty font map with [ctor@Pango2.FontMap.new] and populate it
> only with the fonts you need, using [method@Pango2.FontMap.add_file].
> 
> It is also possible to add custom fonts to the default font map if you
> just want to make some custom font available in addition to the normal
> system fonts. While loading a font from a .ttf or .otf file with
> [method@Pango2.FontMap.add_file] is often the most convenient way to add
> a custom font, it is also possible to load a font from memory by combining
> [ctor@Pango2.HbFace.new_from_hb_face] and hb_face_create().
> 
> Another approach to custom fonts is to draw the glyphs yourself. This
> is possible with [class@Pango2.UserFace]. Such font faces can also be
> added to font maps and used like regular font faces.

So much easier! Big thanks to the Pango development team!

The sys crates in this repo assume the libraries are installed on the system already,
the vendor/ directory contain submodules to the latest cairo, harfbuzz and pango(2).
A makefile in vendor/ makes it easier to compile and install them.