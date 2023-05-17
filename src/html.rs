pub const INDEX: &str = r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charSet="UTF-8" />
    <title>way</title>
    <link rel="stylesheet" href="static/globals.css" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <link rel="icon"
        href="data:image/svg+xml,&lt;svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22>&lt;text y=%22.9em%22 font-size=%2290%22>🤘&lt;/text>&lt;/svg>" />
    <script defer>
        const params = new URLSearchParams(document.location.search.substring(1));
        const ref = params.get("ref");
        window.onload = () => {
            const form = document.getElementById("login-form");
            if (ref) {
                form.setAttribute("action", `/api/login?ref=${ref}`)
            }
        }
    </script>
</head>

<body>
    <html data-custom="data">

    <body>
        <p>
        <div class="welcome">WAY</div>
        </p>
        <div>
            <form id="login-form" method="post" action="api/login">
                <div><input type="text" name="username" placeholder="USERNAME" required="required" /></div>
                <div><input type="password" name="password" placeholder="PASSWORD" required="required" /></div>
                <p>
                    <button type="submit">
                        &nbsp;GO&nbsp;
                    </button>
                </p>
            </form>
        </div>
        <p class="footer">
            Photo by <a href="https://unsplash.com/@etiennegirardet?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Etienne Girardet</a> on <a href="https://unsplash.com/photos/Xh6BpT-1tXo?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>
        </p>
    </body>

    </html>
</body>

</html>
"#;

pub const VERIFIED: &str = r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charSet="UTF-8" />
    <title>way</title>
    <link rel="stylesheet" href="static/globals.css" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <link rel="icon"
        href="data:image/svg+xml,&lt;svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22>&lt;text y=%22.9em%22 font-size=%2290%22>🤘&lt;/text>&lt;/svg>" />
</head>

<body>
    <html data-custom="data">

    <body>
        <div class="welcome">VERIFIED</div>
        <div class="logout-button">
            <p class="dashboard-link"></p>
            <p><a href="/api/logout">
                <button>
                    &nbsp;BYE&nbsp;
                </button></a></p>
        </div>
        <p class="footer">
            Photo by <a href="https://unsplash.com/@etiennegirardet?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Etienne Girardet</a> on <a href="https://unsplash.com/photos/Xh6BpT-1tXo?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>
        </p>
    </body>

    </html>
</body>

</html>
"#;