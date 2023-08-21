use html_node::{
    typed::{elements::*, html},
    Node,
};

fn head() -> Node {
    html!(
        <head>
            <title>Home</title>
            <script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous"></script>
            <meta name="htmx-config" content="{\"useTemplateFragments\": true}" />
            <link rel="stylesheet" href="static/out.css">
        </head>
    )
}

pub fn layout(node: Node) -> Node {
    return html!(
        <html>
            {head()}
            <body>
                {header()}
                {node}
            </body>
       </html>
    );
}

fn header() -> Node {
    html!((hx)
        <header>
            <nav>
                <a href="/todo">Todo</a>
            </nav>
        </header>
    )
}
