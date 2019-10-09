# Diario

Un sistema de blog dinámico escrito en Rust.

* Rust
* Panel de administración
* Formulario de contacto
* API de consulta basada en la web semántica

# Requisitos

Para que Diario funcione se necesita

* Sistema Linux
* Rust (nightly)
* PostgreSQL

# Instalación

* Configura el fichero Rocket.toml (postgresql y gmail)
* `docker-compose up`

# Editor

El editor soporta edición del código fuente. Para ciertas cosas es imprescindible:

* Para insertar código matemático usa `[latex][/latex]`
* Para insertar código de programación usa `<pre><code class="language-XXX">CODIGO TAL CUAL, CON SALTOS DE LINEA Y TODO</code></pre>`

Los tags se insertan separados por comas. La fecha de publicación es aquella en la que su estado pasó a ser Published por primera vez

# Backup/Restore

```
ansible-playbook backup.yml -i 192.168.0.157, --ask-pass
```
