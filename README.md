# temperature-server-exercise
Questo è una esercitazione di raccoglimento e visualizzazione delle temperature delle aule di una scuola. Utilizza le seguenti tecnologie:
* Angular + ngx-chart + nginx per il frontend
* Rust + chrono + clap per il backend e temperature-collector
* Redis come DB
* Python + socket + argparse per fake-sensor
* Docker + Docker swarm + Docker compose per un deploy sperimentale
Non è progettato per essere usato realmente. Ho preso spunto dalla seconda prova dell'esame di maturità (italiana) 2018 informatica e telecomunicazioni articolazioni telecomunicazioni (ITTL).
http://www.istruzione.it/esame_di_stato/201718/Istituti%20tecnici/Ordinaria/I177_ORD18.pdf

## I componenti
Il progetto è composto da quattro componenti:
* Frontend si occupa di servire la webapp.
* Backend si occupa di servire i dati delle temperature
* Redis è un database NOSQL che si occupa di salvare i dati delle temperature
* Temperature-collector si occupa di raccogliere i dati inviati dal sensore e di girarli al database
* Fake-sensor è un sensore virtuale che produce le temperature e che le invia al temperature-collector.
## Come configurare
Il **frontend** può essere configurato modificando il file `frontend/frontend/src/assets/config.json`. Attualmente è possibile configurare solo il url di base del backend. Questo parametro viene configurato in automatico da docker build.
Il **backend** può essere configurato modificato le opportune proprietà in `tse-backend/tsebackend/Rocket.toml`. È possibile configurando passandogli delle variabili ambienti del tipo `ROCKET_nome_proprietà` come `ROCKET_REDIS_URL`.
**temperature-collector** può essere configurato in vari modi: passandogli determinate opzioni a linea di comando, passandogli delle variabili ambiente o modificando il file di configurazione `temperature-collector/temperaturecollector/conf.toml`.
**fake-sensor** è configurabile solo con le opzioni a linea di comando.
## Eseguire il progetto in locale senza docker
### Requisiti
* angular-cli installato
* python3
* rust nightly + cargo. È consigliato installare cargo e rust con rustup. 
* Redis in esecuzione
### Istruzioni
* In `tse-backend/tsebackend` eseguire `cargo run`
* In `frontend/frontend` eseguire `ng serve`
* In `temperature-collector/temperature-collector` eseguire `cargo run`
* In `fake-sensor` eseguire `python3 fake-sensor.py`. È possibile eseguire più volte questo comando passando a ciascuna esecuzione un `--sensor-id` diverso per simulare la presenza di più sensori
* Adesso è possibile visualizzare il progetto con `http://localhost:4200`
## Eseguire il progetto con docker
`
cd tse-backend
docker build -t tse-backend:latest .
cd ../frontend
docker build --arg enable_proxy_pass=true --arg backend_endpoint=backend:8000 -t tse-frontend:proxy_pass
cd ../temperature-collector
docker build -t tse-temperature-collector:latest .
cd ../fake-sensor
docker build -t tse-fake-sensor:latest .
cd ../
docker-compose up
`
PS: non ho verificato che queste istruzioni funzionino. Forse richiede la presenza di un docker swarm o la presenza di un registro.
