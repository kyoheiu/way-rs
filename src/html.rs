pub const INDEX: &str = r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charSet="UTF-8" />
    <title>way</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap');

        body {
             background: rgb(238,174,202);
        background: radial-gradient(circle, rgba(238,174,202,1) 0%, rgba(148,187,233,1) 100%); 
        }
        
        form {
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 30px;
        }
        
        input {
            border: 0px;
            border-radius: 5px;
            margin: 5px;
        }
        
        button {
            color: #fff;
            background-color: #666;
            border: 1px solid #666;
            border-radius: 5px;
        }
        
        .welcome {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 10em;
            color: #666;
            text-align: center;
        }
        
        .logout-button {
            text-align: center;
        }
    </style>
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
        <div class="welcome">way</div>
        </p>
        <div>
            <form id="login-form" method="post" action="api/login">
                <div><input type="text" name="username" placeholder="USERNAME" required="required" /></div>
                <div><input type="password" name="password" placeholder="PASSWORD" required="required" /></div>
                <p>
                    <button type="submit">
                        🚀
                    </button>
                </p>
            </form>
        </div>
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
    <title>suica</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap');

        body {
             background: rgb(238,174,202);
        background: radial-gradient(circle, rgba(238,174,202,1) 0%, rgba(148,187,233,1) 100%); 
        }
        
        form {
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 30px;
        }
        
        input {
            border: 0px;
            border-radius: 5px;
            margin: 5px;
        }
        
        button {
            color: #fff;
            background-color: #666;
            border: 1px solid #666;
            border-radius: 5px;
        }
        
        .welcome {
            font-family: 'Bebas Neue', sans-serif;
            font-size: 10em;
            color: #666;
            text-align: center;
        }
        
        .logout-button {
            text-align: center;
        }
    </style>
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <link rel="icon"
        href="data:image/svg+xml,&lt;svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22>&lt;text y=%22.9em%22 font-size=%2290%22>🤘&lt;/text>&lt;/svg>" />
</head>

<body>
    <html data-custom="data">

    <body>
        <div class="welcome">VERIFIED!</div>
        <div class="logout-button">
            <p class="dashboard-link"></p>
            <p><a href="/logout">
                <button>
                    ✋
                </button></a></p>
        </div>
    </body>

    </html>
</body>

</html>
"#;
