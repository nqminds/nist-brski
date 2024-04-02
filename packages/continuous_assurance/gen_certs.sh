openssl req -x509 -sha256 -nodes -days 365 -subj "/CN=Registrar" -newkey ec:<(openssl ecparam -name prime256v1) -keyout registrar.key -out registrar.crt -addext "subjectAltName=DNS:registrar" -addext "basicConstraints=critical,CA:FALSE"
openssl req -x509 -sha256 -nodes -days 365 -subj "/CN=Router" -newkey ec:<(openssl ecparam -name prime256v1) -keyout router.key -out router.crt -addext "subjectAltName=DNS:router" -addext "basicConstraints=critical,CA:FALSE"