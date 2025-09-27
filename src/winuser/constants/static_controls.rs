use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateWindow, WM_DRAWITEM};

/// A simple rectangle and left-aligns the text in the rectangle. The text is formatted before it
/// is displayed. Words that extend past the end of a line are automatically wrapped to the
/// beginning of the next left-aligned line. Words that are longer than the width of the control
/// are truncated.
pub const SS_LEFT: UINT = 0x00000000;

/// A simple rectangle and centers the text in the rectangle. The text is formatted before it is
/// displayed. Words that extend past the end of a line are automatically wrapped to the beginning
/// of the next centered line. Words that are longer than the width of the control are truncated.
pub const SS_CENTER: UINT = 0x00000001;

/// A simple rectangle and right-aligns the text in the rectangle. The text is formatted before it
/// is displayed. Words that extend past the end of a line are automatically wrapped to the
/// beginning of the next right-aligned line. Words that are longer than the width of the control
/// are truncated.
pub const SS_RIGHT: UINT = 0x00000002;

/// An icon to be displayed in the dialog box. If the control is created as part of a dialog box,
/// the text is the name of an icon (not a filename) defined elsewhere in the resource file. If the
/// control is created via [`CreateWindow`] or a related function, the text is the name of an icon
/// (not a filename) defined in the resource file associated with the module specified by the
/// `instance` parameter to [`CreateWindow`]. The icon can be an animated cursor. The style ignores
/// the [`CreateWindow`] parameters `width` and `height`; the control automatically sizes itself to
/// accommodate the icon. As it uses the [`LoadIcon`] function, the [`SS_ICON`] style can load only
/// icons of dimensions [`SM_CXICON`] and [`SM_CYICON`]. This restriction can be bypassed by using
/// the [`SS_REALSIZEIMAGE`] style in addition to [`SS_ICON`]. If an icon cannot be loaded through
/// [`LoadIcon`], an attempt is made to load the specified resource as a cursor using
/// [`LoadCursor`]. If that too fails, an attempt is made to load from the device driver using
/// [`LoadImage`].
pub const SS_ICON: UINT = 0x00000003;

/// A rectangle filled with the current window frame color. This color is black in the default
/// color scheme.
pub const SS_BLACKRECT: UINT = 0x00000004;

/// A rectangle filled with the current screen background color. This color is gray in the default
/// color scheme.
pub const SS_GRAYRECT: UINT = 0x00000005;

/// A rectangle filled with the current window background color. This color is white in the default
/// color scheme.
pub const SS_WHITERECT: UINT = 0x00000006;

/// A box with a frame drawn in the same color as the window frames. This color is black in the
/// default color scheme.
pub const SS_BLACKFRAME: UINT = 0x00000007;

/// A box with a frame drawn with the same color as the screen background (desktop). This color is
/// gray in the default color scheme.
pub const SS_GRAYFRAME: UINT = 0x00000008;

/// A box with a frame drawn with the same color as the window background. This color is white in
/// the default color scheme.
pub const SS_WHITEFRAME: UINT = 0x00000009;

/// A simple rectangle and displays a single line of left-aligned text in the rectangle. The text
/// line cannot be shortened or altered in any way. Also, if the control is disabled, the control
/// does not gray its text.
pub const SS_SIMPLE: UINT = 0x0000000B;

/// A simple rectangle and left-aligns the text in the rectangle. Tabs are expanded, but words are
/// not wrapped. Text that extends past the end of a line is clipped.
pub const SS_LEFTNOWORDWRAP: UINT = 0x0000000C;

/// The owner of the static control is responsible for drawing the control. The owner window
/// receives a [`WM_DRAWITEM`] message whenever the control needs to be drawn.
pub const SS_OWNERDRAW: UINT = 0x0000000D;

/// A bitmap is to be displayed in the static control. The text is the name of a bitmap (not a
/// filename) defined elsewhere in the resource file. The style ignores the `width` and `height`
/// parameters; the control automatically sizes itself to accommodate the bitmap.
pub const SS_BITMAP: UINT = 0x0000000E;

/// An enhanced metafile is to be displayed in the static control. The text is the name of a
/// metafile. An enhanced metafile static control has a fixed size; the metafile is scaled to fit
/// the static control's client area.
pub const SS_ENHMETAFILE: UINT = 0x0000000F;

/// Draws the top and bottom edges of the static control using the `EDGE_ETCHED` edge style. For
/// more information, see the [`DrawEdge`] function.
pub const SS_ETCHEDHORZ: UINT = 0x00000010;

/// Draws the left and right edges of the static control using the `EDGE_ETCHED` edge style. For
/// more information, see the [`DrawEdge`] function.
pub const SS_ETCHEDVERT: UINT = 0x00000011;

/// Draws the frame of the static control using the `EDGE_ETCHED` edge style. For more information,
/// see the [`DrawEdge`] function.
pub const SS_ETCHEDFRAME: UINT = 0x00000012;

/// A composite style bit that results from using the OR operator on `SS_*` style bits. Can be used
/// to mask out valid `SS_*` bits from a given bitmask. Note that this is out of date and does not
/// correctly include all valid styles. Thus, you should not use this style.
pub const SS_TYPEMASK: UINT = 0x0000001F;

/// Adjusts the bitmap to fit the size of the static control. For example, changing the locale can
/// change the system font, and thus controls might be resized. If a static control had a bitmap,
/// the bitmap would no longer fit the control. This style bit dictates automatic redimensioning of
/// bitmaps to fit their controls. If [`SS_CENTERIMAGE`] is specified, the bitmap or icon is
/// centered (and clipped if needed). If [`SS_CENTERIMAGE`] is not specified, the bitmap or icon is
/// stretched or shrunk. Note that the redimensioning in the two axes are independent, and the
/// result may have a changed aspect ratio. Compare with [`SS_REALSIZEIMAGE`].
pub const SS_REALSIZECONTROL: UINT = 0x00000040;

/// Prevents interpretation of any ampersand (&) characters in the control's text as accelerator
/// prefix characters. These are displayed with the ampersand removed and the next character in the
/// string underlined. This static control style may be included with any of the defined static
/// controls. You can combine [`SS_NOPREFIX`] with other styles. This can be useful when filenames
/// or other strings that may contain an ampersand (&) must be displayed in a static control in a
/// dialog box.
pub const SS_NOPREFIX: UINT = 0x00000080;

/// Sends the parent window [`STN_CLICKED`], [`STN_DBLCLK`], [`STN_DISABLE`], and [`STN_ENABLE`]
/// notification codes when the user clicks or double-clicks the control.
pub const SS_NOTIFY: UINT = 0x00000100;

/// A bitmap is centered in the static control that contains it. The control is not resized, so
/// that a bitmap too large for the control will be clipped. If the static control contains a
/// single line of text, the text is centered vertically in the client area of the control.
pub const SS_CENTERIMAGE: UINT = 0x00000200;

/// The lower right corner of a static control with the SS_BITMAP or SS_ICON style is to remain
/// fixed when the control is resized. Only the top and left sides are adjusted to accommodate a
/// new bitmap or icon.
pub const SS_RIGHTJUST: UINT = 0x00000400;

/// Specifies that the actual resource width is used and the icon is loaded using [`LoadImage`].
/// [`SS_REALSIZEIMAGE`] is always used in conjunction with [`SS_ICON`]. [`SS_REALSIZEIMAGE`] uses
/// [`LoadImage`], overriding the process normally followed under [`SS_ICON`]. It does not load
/// cursors; if [`LoadImage`] fails, no further attempts to load are made. It uses the actual
/// resource width. The static control is resized accordingly, but the icon remains aligned to the
/// originally specified left and top edges of the control. Note that if [`SS_CENTERIMAGE`] is also
/// specified, the icon is centered within the control's space, which was specified using the
/// [`CreateWindow`] parameters `width` and `height`. Compare with [`SS_REALSIZECONTROL`].
pub const SS_REALSIZEIMAGE: UINT = 0x00000800;

/// Draws a half-sunken border around a static control.
pub const SS_SUNKEN: UINT = 0x00001000;

/// The static control duplicates the text-displaying characteristics of a multiline edit control.
/// Specifically, the average character width is calculated in the same manner as with an edit
/// control, and the function does not display a partially visible last line.
pub const SS_EDITCONTROL: UINT = 0x00002000;

/// If the end of a string does not fit in the rectangle, it is truncated and ellipses are added.
/// If a word that is not at the end of the string goes beyond the limits of the rectangle, it is
/// truncated without ellipses. Using this style will force the control's text to be on one line
/// with no word wrap. Compare with [`SS_PATHELLIPSIS`] and [`SS_WORDELLIPSIS`].
pub const SS_ENDELLIPSIS: UINT = 0x00004000;

/// Replaces characters in the middle of the string with ellipses so that the result fits in the
/// specified rectangle. If the string contains backslash (\) characters, [`SS_PATHELLIPSIS`]
/// preserves as much as possible of the text after the last backslash. Using this style will force
/// the control's text to be on one line with no word wrap. Compare with [`SS_ENDELLIPSIS`] and
/// [`SS_WORDELLIPSIS`].
pub const SS_PATHELLIPSIS: UINT = 0x00008000;

/// Truncates any word that does not fit in the rectangle and adds ellipses. Using this style will
/// force the control's text to be on one line with no word wrap. Compare with [`SS_ENDELLIPSIS`]
/// and [`SS_PATHELLIPSIS`].
pub const SS_WORDELLIPSIS: UINT = 0x0000C000;
