(function (url, data) {
    if (typeof fetch !== "undefined") {
        return fetch(
            url,
            {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: data,
            })
            .then(response => response.text());
    } else if (typeof require !== "undefined") {
        return new Promise((resolve, reject) => {
            const scheme = url.startsWith("https") ? "https" : "http";
            const request = require(scheme).request(
                url,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                        "Content-Length": data.length,
                    },
                },
                response => {
                    let body = "";
                    response.setEncoding('utf8');
                    response.on('data', chunk => body = `${body}${chunk}`);
                    response.on('end', () => resolve(body));
                },
            );
            request.on('error', err => reject(err));
            request.write(Buffer.from(data));
            request.end();
        });
    } else {
        throw new Error("unsupported JavaScript runtime");
    }
})
