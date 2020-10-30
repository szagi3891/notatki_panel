import * as React from 'react'
import * as Server from 'react-dom/server'

export const indexContent = (): string => {

    return '<!DOCTYPE html>' + Server.renderToString(
        <html lang="en">
            <head>
                <meta charSet="utf-8"/>
                <title>Panel do zarządzania notatkami</title>
            </head>
            <body>
                <div id="root"></div>
                <script type="text/javascript" src="/static/client.js"></script>
            </body>
        </html>
    );
}