# Via Ethos — Fikir Defteri (Panel PC Vizyonu)

> Bu doküman ideasyon aşamasının çıktısıdır; ARCHITECTURE.md'deki teknik
> mimarinin üzerine gelen ürün/tasarım fikirlerini toplar. Türkçedir çünkü
> ürün sahibinin düşünme dilidir; koda inen kararlar ARCHITECTURE.md'ye
> İngilizce işlenir.

## 1. Konsept: "Uygulama" değil, "Ev Sunağı"

Hedef cihaz: hep açık, dokunmatik bir panel PC (duvarda/masada). Bu,
uygulamayı tıklanıp açılan bir programdan **ambient bir nesneye** çevirir.

Üç mod:

- **Sessiz Mod (varsayılan):** Panel bir saat gibi davranır — büyük saat,
  Günün Mozaiği, Günün Düsturu (felsefi alıntı), Yol şeridi. Uzaktan tek
  bakışta "günüm nasıl gidiyor?" cevaplanır.
- **Dokunma Modu:** Dokununca panel uyanır — Quick Tick'ler büyük
  dokunmatik dairelere dönüşür, gün planı etkileşimli olur. ~30 sn
  hareketsizlikte Sessiz Mod'a geri süzülür.
- **Ritüel Modu:** Focus seansında her şey kaybolur; nefes alan halka +
  niyet. Panel evdeki herkese "odaktayım" diyen bir işarete dönüşür.

Dokunmatik kurallar: min. 48px hedefler, hover'a asla güvenme, parmak
dostu sürükle-bırak, kiosk/tam ekran açılış.

## 2. Gün Planlama: "Günün Mozaiği"

Gün liste değil, **mozaik/vitray** olarak gösterilir:

- Dikey zaman şeridi (06:00–24:00); her plan bloğu Pillar renginde bir
  cam parçası. Boş saatler buzlu/şeffaf.
- Planlama = şeride parmakla blok sürüklemek.
- Tamamlanan blok parlar; yarıda kalan soluklaşır; boşa giden gri değil
  **amber** — ceza değil, bilgi.
- Gün bitince mozaik küçülüp Yol'a bir taş olur: **Yol renkli bir mozaik
  yoldur.** Hangi haftaların hangi Pillar ağırlıklı geçtiği renk
  dokusundan okunur.

## 3. Felsefi Dokunuşlar (süs değil, mekanik)

- **Sabah Niyeti (Premeditatio):** Günün ilk dokunuşunda: "Bugün hangi
  erdeme hizmet edeceksin?" Seçilen erdeme bağlı aksiyonlar o gün hafif
  vurgulanır.
- **Akşam Muhasebesi (Seneca):** Gün sınırında üç soru: Neyi iyi yaptım?
  Nerede zayıftım? Yarına ne kalıyor? Cevaplar günü mühürler, puan
  getirir, yarının planına tohum atar.
- **Günün Düsturu:** Sessiz Mod'da günlük alıntı (Stoacılar; istenirse
  Yunus Emre/Mevlana harmanlı Türkçe küratörlük). Alıntı duruma göre
  seçilir: streak kırıldıysa Amor Fati, yoğun günse itidal temalı.
- **Memento Mori Takvimi:** Hayatın haftaları ızgarası (90 yıl × 52
  hafta); yaşanmış haftalar Pillar renk karışımıyla dolu. Ayda bir gece
  Sessiz Mod bu görünüme döner.
- **Yol Durakları (rütbe yerine):** Talebe → Yolcu → Yoldaş → Usta →
  Bilge. Rozet yağmuru yok; her durak Yol'da tek bir taş kapı.
- **Amor Fati mekaniği:** Kaçan gün "streak kırıldı ✗" değil; "Yol devam
  ediyor" + yolda yargısız bir taş aralığı. Suçluluk yerine devamlılık.

## 4. Renk Sistemi: Renkli ama Gürültüsüz

v2'deki monokrom yön revize edildi; disiplin korunuyor:

- **Renk = anlam.** Her renk bir Pillar'ındır; dekoratif renk yok.
  Örn. Mind = gece mavisi, Body = nar/mercan, Craft = safran/amber,
  Life = adaçayı yeşili — koyu sıcak zeminde **mücevher tonları**
  (vitray etkisi; hep açık ekranda koyu zemin pratik avantaj).
- **Doygunluk = canlılık.** Aktif streak'li Pillar doygun, ihmal edilen
  soluk. "Hayatımın hangi alanı soluyor?" rengin kendisinden okunur.
- **Mevsimsel zemin:** Zemin tonu yıl boyunca çok yavaş kayar (kış:
  derin lacivert, bahar: sıcak grafit). Panel "yaşıyor" hisseder.

## 5. Ton Rehberi: Mistik Ruh, Kusursuz Kullanışlılık

Netleştirilen ilke: **mistik/felsefi atmosfer korunur; "profesyonellik"
icranın kalitesinde ve kullanışlılıkta gösterilir.** Felsefe mekanikte,
içerikte ve dilde yaşar — ama asla kullanım hızının önüne geçmez.

- **Ritüel dili kalır:** Niyet, Mühürle, Düstur, Muhasebe, Yol, Erdem —
  bu sözlük uygulamanın kimliğidir. Ancak her ritüel teriminin arkasında
  net, tek dokunuşluk bir eylem durur: "Mühürle" bir buton olarak
  belirsizlik taşımaz.
- **Kullanışlılık çıtası (sert kurallar):**
  - Bir Quick Tick işaretlemek: 1 dokunuş, 0 gezinme.
  - Odak seansı başlatmak: en fazla 2 dokunuş + isteğe bağlı niyet.
  - Ritüel hiçbir zaman zorunlu bekletme yaratmaz — atmosfer animasyonları
    kesilebilir/atlanabilir, akışı asla kilitlemez.
  - Her ekran tek soruya cevap verir; cevabı 3 saniyede okunamıyorsa
    ekran fazla kalabalıktır.
- **İçerik gücü = kaynaklı küratörlük.** Her alıntı eser + bölüm
  referansıyla verilir (örn. "Marcus Aurelius · Düşünceler, VIII.47").
  Jenerik motivasyon cümleleri ("Bugün harika olacak!") yasaktır.
  Alıntı kitaplığı paketle gelir, temalara etiketlenir, duruma göre seçilir.
- **Mikrometin ilkeleri:** sakin, kısa, fiil odaklı; asla ünlem, asla
  suçlayıcı dil. Kaçan blok "başarısız" değil "ertelendi"dir.
- **Görsel referans çıtası:** manastır mimarisinin sükuneti + lüks saat
  kadranı hassasiyeti. Mistik doku tipografi, renk ve ışıkla kurulur —
  konfeti ve rozet yağmuruyla değil.

## 6. Mimarîye Etkisi

Mevcut mimari (Tauri v2 + Svelte 5 + SQLite, Rust'ta domain) bu vizyonu
taşır. Gerekli eklemeler:

1. **`day_plans` tablosu** — mozaik blokları: tarih, saat aralığı,
   action/task bağı, durum.
2. **`reflections` tablosu** — sabah niyeti (günün erdemi) + akşam
   muhasebesi (üç cevap), güne bağlı.
3. **`quotes` içeriği** — paketle gelen, temalı/etiketli alıntı seti;
   seçim mantığı Rust'ta (duruma göre: amor_fati / itidal / cesaret...).
4. **Idle/presence yöneticisi (Rust)** — Sessiz ↔ Dokunma modu geçişi,
   kiosk/tam ekran, ekran koruma davranışı.
5. Görünümler: Bugün(Panel) / Ritüel / Yol / Memento Mori / Yönetim /
   Ayarlar.
