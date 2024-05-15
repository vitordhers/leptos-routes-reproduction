use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <h1>"Router issue reproduction App"</h1>
        <Router>
            <Routes>
                // this works ok

                // <Route path="/" view=|| view! {<Html lang="en-US" /><Outlet />}>
                //     <Route path="/" view=|| view! {<SimpleNavComponent />}>
                //         { view! {<DeepNestedRouteSelector url=1 />} }
                //         { view! {<DeepNestedRouteSelector url=2 />} }
                //     </Route>
                //     <Route path="/comp3" view=SimpleComponent3 />
                //     <Route path="/comp4" view=SimpleComponent4 />
                // </Route>

                // this one also works ok
                // <EnumerationNestedRoutes />

                // this doesn't work
                // <FragmentNestedRoutes />

                // this doesn't work
                <CollectedNestedRoutes />
            </Routes>
        </Router>
    }
}

#[component()]
fn SimpleNavComponent() -> impl IntoView {
    view! {
        <nav>
            <A href="/comp1">"Component 1"</A>
            <A href="/comp2">"Component 2"</A>
            <A href="/comp3">"Component 3"</A>
            <A href="/comp4">"Component 4"</A>
        </nav>
        <Outlet />
    }
}

#[component()]
fn SimpleComponent1() -> impl IntoView {
    view! {<h1>"This is simple component 1"</h1>}
}

#[component()]
fn SimpleComponent2() -> impl IntoView {
    view! {<h1>"This is simple component 2"</h1>}
}

#[component()]
fn SimpleComponent3() -> impl IntoView {
    view! {<h1>"This is simple component 3"</h1>}
}

#[component()]
fn SimpleComponent4() -> impl IntoView {
    view! {<h1>"This is simple component 4"</h1>}
}

#[component(transparent)]
fn DeepNestedRouteSelector(#[prop(default = 1)] url: i8) -> impl IntoView {
    let path = match url {
        1 => "/comp1",
        _ => "/comp2",
    };

    view! {
        <Route path=path view=move || match url {
            1 => view! {<SimpleComponent1 />},
            _ => view! {<SimpleComponent2 />},
        } />
    }
}

#[component(transparent)]
fn EnumerationNestedRoutes() -> impl IntoView {
    view! {
        <Route path="/" view=|| view! {<Html lang="en-US" /><Outlet />}>
            <Route path="/" view=|| view! {<SimpleNavComponent />}>
                { view! {<DeepNestedRouteSelector url=1 />} }
                { view! {<DeepNestedRouteSelector url=2 />} }
            </Route>
            <Route path="/comp3" view=SimpleComponent3 />
            <Route path="/comp4" view=SimpleComponent4 />
        </Route>
    }
}

#[component(transparent)]
fn FragmentNestedRoutes() -> impl IntoView {
    view! {
        <Route path="/" view=|| view! {<Html lang="en-US" /><Outlet />}>
            <Route path="/" view=|| view! {<SimpleNavComponent />}>
                {Fragment::from_iter(vec![1, 2].into_iter().map(|n: i8| view!{ <DeepNestedRouteSelector url=n /> }))}
            </Route>
            <Route path="/comp3" view=SimpleComponent3 />
            <Route path="/comp4" view=SimpleComponent4 />
        </Route>
    }
}

#[component(transparent)]
fn CollectedNestedRoutes() -> impl IntoView {
    view! {
        <Route path="/" view=|| view! {<Html lang="en-US" /><Outlet />}>
            <Route path="/" view=|| view! {<SimpleNavComponent />}>
                {vec![1, 2].into_iter().map(|n: i8| view!{ <DeepNestedRouteSelector url=n /> }).collect_view()}
            </Route>
            <Route path="/comp3" view=SimpleComponent3 />
            <Route path="/comp4" view=SimpleComponent4 />
        </Route>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
