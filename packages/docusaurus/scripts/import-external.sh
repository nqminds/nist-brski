echo "importing docs.."

cd "docs/build5-docs"

wget "https://raw.githubusercontent.com/nqminds/nist-brski/main/brski-server/README.md" -O "35-BRSKI-demonstrator/20-demo-setup.md"


cp "../../../schemas/README.md" "60-CA-Demo/630-schemas-setup.md"
cp ../../../schemas/*.png "60-CA-Demo/"

cp "../../../nist_policy/README.md" "60-CA-Demo/640-nist_policy.md"

cp "../../../nist_vc_rest_server/README.md" "60-CA-Demo/650-nist_vc_rest_server.md"
cp ../../../nist_vc_rest_server/*.png "60-CA-Demo/"

cp "../../../nist_registrar_server/README.md" "60-CA-Demo/660-nist_registrar_server.md"
cp ../../../nist_registrar_server/*.png "60-CA-Demo/"

cp "../../../registrar_demo_app/README.md" "60-CA-Demo/670-registrar_demo_app.md"
cp ../../../registrar_demo_app/*.png "60-CA-Demo/"


wget "https://raw.githubusercontent.com/nqminds/brski/main/README.md" -O "40-BRSKI developer/10-BRSKI-build-instructions.md"


wget "https://raw.githubusercontent.com/nqminds/brski/main/docs/voucher.md" -O "40-BRSKI-developer/docs/voucher.md"
wget "https://raw.githubusercontent.com/nqminds/brski/main/docs/brski.md" -O "40-BRSKI-developer/docs/brski.md"
wget "https://raw.githubusercontent.com/nqminds/brski/main/docs/usage.md" -O "40-BRSKI-developer/docs/usage.md"
wget "https://raw.githubusercontent.com/nqminds/brski/main/docs/array.md" -O "40-BRSKI-developer/docs/array.md"
