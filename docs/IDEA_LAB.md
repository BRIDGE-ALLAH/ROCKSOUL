# THE IDEA LAB — WILD IDEAS, REAL GUARDRAILS

**Think with chaos. Act with riot.**

Ini bukan backlog janji. Ini lab ide. Setiap konsep punya level: *build*, *prototype*, atau *wild bet*. Ide boleh liar; bukti tetap wajib.

## 1. THE RIOT PIT — agent yang berani debat

Jangan bikin semua agent jadi yes-man.

Bikin crew dengan mandat berbeda:
- **Builder:** cari cara paling masuk akal untuk membuat hasil.
- **Skeptic:** cari asumsi rapuh, counterexample, dan lubang logika.
- **Researcher:** cari bukti dan sumber yang menentang maupun mendukung.
- **Operator:** cek dependensi, urutan langkah, biaya, dan risiko eksekusi.
- **Editor:** rapikan hasil jadi artifact yang bisa dipakai.
- **Guardian:** pastikan izin, data, dan batas tindakan tidak dilanggar.

Setiap role punya input dan output terstruktur. Mereka tidak perlu saling melihat semua konteks. Synthesis harus menunjukkan konflik yang belum selesai, bukan memaksa suara bulat.

**Status:** prototype setelah mission harness stabil.

## 2. CHAOS CANVAS — ide boleh bercabang

Alih-alih satu chat linear, setiap ide bisa bercabang jadi beberapa hipotesis:
- "Apa asumsi yang kita pakai?"
- "Kalau kebalikannya benar, apa yang berubah?"
- "Bagaimana desainer, engineer, musisi, dan operator melihat masalah yang sama?"
- "Apa eksperimen termurah yang bisa membunuh ide buruk?"

Tiap cabang punya parent, sumber, bukti, biaya, dan hasil. Cabang bisa digabung, diarsipkan, atau dipromosikan jadi mission.

**Status:** prototype UI awal.

## 3. PROOF-CARRYING CREATION — karya bawa kuitansinya

Setiap artifact bisa diekspor bersama evidence manifest:
- file dan hash,
- versi mission dan acceptance criteria,
- referensi/sumber,
- tool calls yang relevan,
- test output,
- approval,
- batasan dan pertanyaan terbuka.

Bayangkan mengirim prototype sambil mengirim "black box flight recorder"-nya. Orang lain bisa mengerti dari mana hasilnya datang dan apa yang belum diverifikasi.

**Status:** build early; ini harus jadi DNA produk.

## 4. PROJECT TIME MACHINE — putar balik sejarah proyek

Bukan hanya undo file. Putar balik konteks:
- kapan keputusan dibuat,
- eksperimen mana yang gagal,
- siapa/agent mana yang mengubah artifact,
- kenapa arah proyek bergeser,
- apa dampaknya jika keputusan tertentu tidak dibuat.

Gunakan event log dan snapshots. Jangan mengubah ulang fakta historis; buat correction event yang menjelaskan perbaikannya.

**Status:** build foundation.

## 5. WORKTREE ORCHESTRA — satu agent, satu studio

Setiap agent yang mengerjakan kode mendapat Git worktree sendiri. Boleh paralel, tapi tidak saling menginjak:
- branch dan workspace terisolasi,
- budget/timeout terpisah,
- acceptance criteria yang spesifik,
- tes jalan sebelum merge,
- conflict masuk ke merge lane yang transparan.

Bisa dibayangkan seperti beberapa studio rekaman: tiap band merekam track sendiri; engineer yang mencampur hasil akhirnya.

**Status:** prototype setelah local mission runner.

## 6. REALITY LOOM — satu misi, banyak permukaan

Satu alur kerja dapat melompat antara:
- file manager dan filesystem,
- browser,
- terminal,
- editor,
- API,
- media workspace.

Gunakan accessibility tree/semantic UI lebih dulu, screenshot/vision sebagai fallback. Setiap action harus mengambil ulang state setelah klik/ketik; jangan menebak bahwa tombol berhasil ditekan.

**Status:** research + prototype. Ini area sulit dan wajib benchmark hybrid GUI/CLI.

## 7. MEMORY THAT HAS A CONSCIENCE

Memory bukan semua yang pernah dikatakan user.

Simpan tipe yang berbeda:
- episode: apa yang benar-benar terjadi,
- fact: sesuatu yang punya sumber dan bisa dikoreksi,
- interpretation: kesimpulan yang masih bisa berubah,
- procedure: cara menyelesaikan sesuatu,
- preference: preferensi yang dikendalikan user.

Memory yang dipakai ulang harus punya scope, provenance, tanggal, confidence, dan tombol koreksi/hapus. Sistem harus bisa berkata "saya tidak yakin ini masih berlaku".

**Status:** build event ledger, prototype consolidation.

## 8. MODEL DJ — mix model, jangan fanboy

Pilih model seperti DJ memilih track:
- model lokal untuk data sensitif atau tugas ringan,
- cloud model untuk tugas yang memang membutuhkannya,
- model cepat untuk klasifikasi/format,
- model kuat untuk synthesis kompleks,
- verifier berbeda untuk cek hasil.

Routing berdasarkan data policy dulu, baru kualitas yang diukur, latency, dan harga. Tampilkan alasan pemilihan model. Tidak ada model yang otomatis jadi raja.

**Status:** prototype setelah provider contract dan eval harness.

## 9. COUNTERFACTUAL GARAGE

Sebelum deploy atau mengubah konfigurasi, buat mini simulasi:
- "Apa yang rusak kalau service A mati?"
- "Bagaimana kalau model tidak tersedia?"
- "Apa efek jika egress dimatikan?"
- "Bagaimana rollback bila migrasi gagal?"
- "Apa hasil dari strategi B dibanding A?"

Gunakan test environments, fixtures, dan simulation adapters. Jangan pura-pura memprediksi dunia nyata; tunjukkan asumsi dan batas model simulasi.

**Status:** wild bet; mulai dari repo/build/deploy scenario.

## 10. SKILL FORGE — skill yang bisa dites

Skill bukan cuma prompt file. Setiap skill punya:
- `SKILL.md` untuk instruksi manusia/agent,
- manifest untuk input/output dan capability needs,
- test fixtures,
- expected behavior,
- compatibility metadata,
- changelog dan provenance.

Skill yang minta akses jaringan atau file harus menyatakannya. Skill tidak boleh memberikan izin ke dirinya sendiri.

**Status:** prototype; kompatibilitas lintas-agent jadi nilai jual.

## 11. HUMAN SIGNAL METER

Ukur bukan cuma output AI, tapi kualitas kolaborasinya:
- berapa kali manusia harus membetulkan,
- approval mana yang dibatalkan,
- klaim mana yang tidak punya bukti,
- berapa banyak keputusan yang dibantu vs diambil alih,
- apakah user makin paham atau makin bergantung.

Bukan surveillance score. Metrik lokal, opt-in, transparan, dan bisa dihapus.

**Status:** wild bet / research.

## 12. AGILITY BENCH — ukur kelincahan, bukan slogan

Bangun benchmark internal untuk:
- adaptasi saat requirement berubah,
- transfer cara kerja ke repo/domain baru,
- ketahanan memory lintas sesi,
- recovery setelah tool gagal,
- kualitas hasil lintas GUI + CLI + file,
- kalibrasi: tahu kapan tidak tahu,
- kemampuan menerima koreksi manusia.

Laporkan model, harness, tool versions, task set, biaya, waktu, success, intervention, dan traces. Jangan menyebut angka ini "AGI score".

**Status:** build sejak hari pertama.

## 13. THE CREATIVE COLLIDER

Ide liar yang lebih ambisius: satu workspace untuk code, visual, sound, video, research, dan physical-world prototypes. Brief yang sama bisa menghasilkan:
- prototype UI,
- music/video storyboard,
- simulation,
- code,
- explainer,
- evidence package.

Bukan berarti semua harus dihasilkan oleh satu model. Rocksoul mengatur crew dan tools yang paling cocok.

**Status:** long-term research.

## What we build first

| Priority | Concept | Reason |
|---|---|---|
| P0 | Project Time Machine foundation | Tanpa state dan event history, agent lupa dan tak bisa dipertanggungjawabkan. |
| P0 | Proof-Carrying Creation | Pembeda utama dari chatbot/demo biasa. |
| P0 | Agility Bench | Kita perlu tahu mana yang benar-benar membaik. |
| P1 | Chaos Canvas | Memberi ruang ide bercabang tanpa mengorbankan struktur. |
| P1 | Worktree Orchestra | Membuka parallel work dengan risiko konflik lebih rendah. |
| P1 | Skill Forge | Membuat workflow portable dan bisa diuji. |
| P2 | Riot Pit | Multi-agent baru masuk setelah single-agent loop and verifier terbukti. |
| P2 | Reality Loom | Sangat menarik, tetapi harus dibangun di atas permission + trace + eval. |
| P3 | Counterfactual Garage / Creative Collider | Eksperimen jangka panjang. |

**Rule:** one wild idea per release, one measurable hypothesis per experiment, one honest result.