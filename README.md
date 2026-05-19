# Tutorial 3: WebChat using Yew
 
Pada tutorial ini, kita membangun aplikasi **WebChat berbasis browser** menggunakan **Yew** — framework frontend Rust yang dikompilasi ke WebAssembly (WASM). Berbeda dari tutorial sebelumnya yang berbasis konsol, kini komunikasi real-time dilakukan melalui tampilan grafis di browser.
 
Referensi utama: [Let's Build a WebSockets Project with Rust and Yew 0.19](https://blog.devgenius.io/lets-build-a-websockets-project-with-rust-and-yew-0-19-60720367399f)
 
---
 
## Experiment 3.1: Original Code
 
### Deskripsi
 
Pada bagian ini, saya mempelajari dan menjalankan dua repositori:
 
1. **YewChat** — client WebChat yang ditulis dengan Rust/Yew, dikompilasi ke WASM
2. **SimpleWebsocketServer** — server WebSocket yang ditulis dengan Node.js/TypeScript
### Cara Menjalankan
 
**Server (JavaScript/TypeScript):**
```bash
cd SimpleWebsocketServer
npm install
npm start
# Server berjalan di ws://localhost:8080
```
 
**Client (Rust/Yew):**
```bash
cd YewChat
npm install
wasm-pack build --target web --out-dir pkg --release
npm start
# Aplikasi berjalan di http://localhost:8000
```
 
### Permasalahan yang Ditemui
 
Beberapa masalah muncul selama proses setup:
 
1. **`spawn EINVAL`** — Webpack plugin `@wasm-tool/wasm-pack-plugin` gagal memanggil subprocess di Windows. Solusi: nonaktifkan plugin di `webpack.config.js` dan jalankan `wasm-pack build` secara manual.
2. **`module.run_app is not a function`** — `bootstrap.js` memanggil fungsi yang tidak ter-export karena perbedaan API antara Yew 0.19 dan 0.21. Solusi: ubah `bootstrap.js` menjadi `import init from './pkg/yewchat.js'; init();` dan gunakan `#[wasm_bindgen(start)]` di Rust.
3. **Versi `wasm-bindgen` tidak kompatibel** — Versi lama tidak cocok dengan Rust terbaru. Solusi: update semua dependency di `Cargo.toml` ke versi terbaru.
4. **Pesan hanya tampil satu** — Closure di `use_effect_with` menutup nilai state yang lama sehingga setiap pesan baru menimpa yang lama. Solusi: gunakan `Rc<RefCell<Vec<ChatMessage>>>` agar closure selalu mengakses list pesan yang sama di memori.
### Hasil
 
Aplikasi berhasil dijalankan dengan tampilan login dan chat yang fungsional:
 
- Halaman login dengan input username
- Halaman chat dengan sidebar daftar user online
- Pesan real-time antara beberapa tab browser/user berbeda
> **Screenshot:** 
![alt text](image.png)
![alt text](image-1.png)
![alt text](image-2.png)
![alt text](image-3.png)
![alt text](image-4.png)
![alt text](image-5.png)
 
---