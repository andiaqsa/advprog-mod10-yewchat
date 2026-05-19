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


## Experiment 3.2: Be Creative!
 
### Deskripsi
 
Pada bagian ini, saya menambahkan berbagai kreativitas pada webclient untuk membuat pengalaman chat yang lebih menarik dan personal. Inspirasi dari pernyataan WEF bahwa kreativitas adalah kunci untuk bersaing di era AI dalam dunia kerja masa depan.
 
Referensi: [WEF — Creativity as Key Workforce Skill](https://www.weforum.org/agenda/2020/11/ai-automation-creativity-workforce-skill-fute-of-work/)
 
### Perubahan yang Dilakukan
 
**1. Desain UI Dark Mode Modern**
 
Tampilan dirancang ulang dengan tema gelap menggunakan Tailwind CSS. Warna utama biru-gelap (`gray-900`, `gray-800`) memberikan kesan profesional dan nyaman untuk mata saat digunakan lama.
 
**2. Halaman Login yang Elegan**
 
Halaman login dibuat dengan card terpusat di tengah layar, dilengkapi:
- Ikon emoji 💬 sebagai identitas visual aplikasi
- Teks sambutan dalam Bahasa Indonesia
- Input field dengan efek focus border biru
- Tombol "Masuk ke Chat →" dengan hover effect
**3. Sidebar Daftar User Online**
 
Sidebar kiri menampilkan daftar semua user yang sedang terhubung secara real-time, dengan:
- Indikator titik hijau (●) menandakan status online
- Update otomatis saat user bergabung atau keluar
**4. Bubble Chat Dua Arah**
 
Pesan dibedakan secara visual:
- **Pesan sendiri** — bubble biru, rata kanan, tanpa nama pengirim
- **Pesan orang lain** — bubble abu-abu gelap, rata kiri, dengan nama pengirim di atas
**5. Teks Antarmuka Bahasa Indonesia**
 
Semua teks UI menggunakan Bahasa Indonesia ("Ketik pesan...", "Kirim", "Logged in as", "Online") untuk pengalaman yang lebih personal dan lokal.
 
**6. Routing Halaman dengan Yew Router**
 
Menggunakan `yew-router` untuk navigasi SPA:
- `/` → halaman Login
- `/chat` → halaman Chat
- Username disimpan di `sessionStorage` dan diambil saat masuk halaman chat
### Penjelasan Teknis Kreativitas
 
Kreativitas pada bagian ini tidak hanya terbatas pada estetika visual, tetapi juga pada **arsitektur kode**. Saya memilih pendekatan yang bersih dengan memisahkan komponen ke dalam modul terpisah:
 
```
src/
  lib.rs              — entry point WASM
  components/
    mod.rs            — daftar modul
    app.rs            — routing utama
    login.rs          — halaman login
    chat.rs           — halaman chat
    types.rs          — struktur data pesan
```
 
Pemisahan ini membuat kode lebih mudah dibaca, diuji, dan dikembangkan — prinsip yang sama pentingnya dengan kreativitas visual dalam rekayasa perangkat lunak.
 
### Hasil
 
> **Screenshot:** 

![alt text](image-6.png)

![alt text](image-7.png)

![alt text](image-8.png)

![alt text](image-9.png)

![alt text](image-10.png)
 
---