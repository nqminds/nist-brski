openssl req -x509 -sha256 -nodes -days 365 -subj "/CN=Application" -newkey ec:<(openssl ecparam -name prime256v1) -keyout app.key -out app.crt -addext "subjectAltName=IP:192.168.50.112"