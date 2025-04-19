use crate::icons::BIFiles;
use yew::{html, virtual_dom::VNode};

/// Links to the Bootstrap CSS CDN
pub fn include_cdn() -> VNode {
    html! {
        <link
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css"
            rel="stylesheet"
            integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3"
            crossorigin="anonymous"
        />
    }
}

/// Alias for [`include_cdn_js_bundled()`] for compatibility.
///
/// Use [`include_cdn_js_unbundled()`] instead. The bundled version of Bootstrap
/// includes Popper, which like Yew, assumes complete control of the DOM, and
/// causes rendering glitches.
#[inline(always)]
#[deprecated = "Alias for include_cdn_js_bundled(). Migrate to include_cdn_js_unbundled()."]
pub fn include_cdn_js() -> VNode {
    #[allow(deprecated)]
    include_cdn_js_bundled()
}

/// Links to the Bootstrap JS on a CDN, using the non-bundled version.
/// *You should use this version of the JS.*
///
/// The non-bundled version *doesn't* include Popper, so it won't conflict with
/// Yew. `yew-bootstrap` includes `popper-rs`, which is properly integrated with
/// Yew.
///
/// You won't be able to use `data-` attributes for [dropdown menus][0] and
/// [tooltips][1] with the non-bundled version of the JS. Instead, you should
/// use the [`Dropdown`][] and [`Tooltip`][] components, which are properly
/// integrated with Yew.
///
/// [0]: https://getbootstrap.com/docs/5.3/components/dropdowns/
/// [1]: https://getbootstrap.com/docs/5.3/components/tooltips/
/// [`Dropdown`]: crate::component::Dropdown
/// [`Tooltip`]: crate::component::Tooltip
pub fn include_cdn_js_unbundled() -> VNode {
    html! {
        <>
            <link
                data-trunk={"true"}
                rel="copy-file"
                href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.min.js.map"
            />
            <script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.min.js"
                data-trunk={"true"}
                integrity="sha256-cMPWkL3FzjuaFSfEYESYmjF25hCIL6mfRSPnW8OVvM4="
                crossorigin="anonymous"
            >
            </script>
        </>
    }
}

/// Includes the "bundled" version of Bootstrap's JavaScript, which includes
/// Popper.
/// 
/// **This is buggy, and included for backward compatibility only. You should
/// migrate to [`include_cdn_js_unbundled()`].**
///
/// Popper assumes it has complete control of the DOM, and so does Yew. So
/// Popper will conflict with Yew, causing intermittent rendering glitches when
/// building [dropdowns][0] and [tooltips][1] with `data-` attributes.
///
/// `popper-rs` and our native [`Dropdown`][] and [`Tooltip`][] components fully
/// integrate Popper with Yew's control of the DOM. These components don't work
/// correctly with the bundled version of the JS.
///
/// [0]: https://getbootstrap.com/docs/5.3/components/dropdowns/
/// [1]: https://getbootstrap.com/docs/5.3/components/tooltips/
/// [`Dropdown`]: crate::component::Dropdown
/// [`Tooltip`]: crate::component::Tooltip
#[deprecated = "Migrate to include_cdn_js_unbundled()"]
pub fn include_cdn_js_bundled() -> VNode {
    html! {
        <>
            <link
                data-trunk={"true"}
                rel="copy-file"
                href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js.map"
            />
            <script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js"
                data-trunk={"true"}
                integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3"
                crossorigin="anonymous"
            >
            </script>
        </>
    }
}

/// Inserts the bootstrap CSS directly into the content of the page
pub fn include_inline() -> VNode {
    html! {
        <style>
            {include_str!("bootstrap-5.1.3.min.css")}
        </style>
    }
}

/// Include the Bootstrap Icons CDN
#[inline(always)]
#[deprecated = "Use icons::BIFiles::cdn() instead"]
pub fn include_cdn_icons() -> VNode {
    BIFiles::cdn()
}
