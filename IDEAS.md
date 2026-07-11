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
- **Revizyon (canlı yön):** Ürün sahibinin kişiliğine uygun olarak palet
  canlandırıldı — koyu menekşe/indigo zemin üzerinde yavaş hareket eden
  aurora ışıkları, parlak mücevher tonları (Mind #7FB2FF, Body #FF7E67,
  Craft #FFBE55, Life #6FDD9A). Sessiz Mod ekranı bir yazılım paneli
  gibi değil, **duvarda asılı bir tablo gibi** kompoze edilir: merkezde
  büyük saat, ortada yatay vitray friz (Günün Mozaiği), altta merkezde
  serif düstur. Arayüz dili **İngilizce**dir (Virtue of the day, The
  Path, honored/deferred); felsefi sözlük İngilizce'de de korunur.

## 7. Yaşayan Manzara + İç Hava Durumu

İkinci revizyon: statik aurora yerine **saate göre gerçekten değişen,
prosedürel olarak çizilen bir manzara** (Canvas): şafak → gündüz →
günbatımı → gece döngüsü; güneş/ay bir yay üzerinde hareket eder, yıldızlar
gece belirir, dağ silüetleri günün ışığından boyanır. Panel artık bir
ekran değil, gün boyunca kendi kendine değişen bir tablo.

Bunun üzerine, ürün sahibinin ifade etmek istediği kişisel boyutu
karşılayan bir katman: **İç Hava Durumu (Inner Weather).**

- Kullanıcı günlük (veya istediği an) ruh halini dört durumdan biriyle
  işaretler: **Clear** (berrak), **Radiant** (parlak/taşkın), **Heavy**
  (ağır/düşük), **Stormy** (fırtınalı/yoğun dalgalanma). Bu, klinik bir
  "mood tracker" değil — manzaranın atmosferine işleyen sanatsal bir
  ifade katmanı.
- Seçim, manzaranın **ışığını ve doygunluğunu** değiştirir (Radiant:
  daha canlı ve parlak; Heavy: soluk ve loş; Stormy: fırtına bulutları
  ve düşük kontrast) — sayı veya grafikle değil, tablo'nun kendi
  havasıyla yansıtılır.
- **Yargısız kayıt:** Fırtınalı bir gün "kötü gün" değil, Yol üzerinde
  farklı renkte bir taştır — geçmişe bakınca ruh hali örüntülerini
  görmek mümkün olur (hangi mevsimde, hangi Pillar yoğunluğunda hangi
  hava daha sık?) ama bu bir performans skoru değildir.
- Mekanik olarak: `reflections` tablosuna `inner_weather` alanı eklenir
  (clear/radiant/heavy/stormy), günün manzarasını o gün boyunca etkiler.
  Zorunlu değildir; işaretlenmezse manzara yalnızca saate göre değişir.
- Mockup'ta demo amaçlı bir saat kaydırıcısı ve hava durumu seçici var
  (yalnızca önizleme içindir, üretim arayüzünde yer almaz) — gerçek
  üründe saat otomatik akar, iç hava durumu ayrı bir ince kontrolden
  (ör. günlük giriş anında) seçilir.

## 8. Van Gogh Fırça İşi

Üçüncü revizyon: düz/vektörel manzara yerine gökyüzü artık gerçek fırça
darbeleriyle "boyanıyor" (Canvas üzerinde girdap alanı / flow-field
tekniğiyle). Referans doğrudan Starry Night'ın kendisi değil, onun
resim dilinin (kıvrımlı gökyüzü, ışıldayan yıldız haleleri, koyu servi
silüeti) bir yorumu.

- **Girdap alanı (flow field):** gökyüzündeki her fırça darbesi, iki
  görünmez girdap merkezinin ve dalgalı bir zemin akıntısının
  toplamından yön alır — rastgele değil, tutarlı bir "rüzgâr" hisleri.
- **İç Hava Durumu artık kıvrımın kendisini de değiştiriyor:** Clear
  sakin/uzun darbeler; Radiant parlak/altın-turuncu ve daha yoğun
  girdap; Heavy yavaş/soluk/seyrek darbe; Stormy çılgın frekans, yüksek
  genlik, koyu mor-lacivert palet. Yani ruh hali sayı değil,
  **fırçanın davranışı** olarak ifade ediliyor.
- **Yıldızlar** nokta değil, ışıldayan halka/hale darbeleri (Starry
  Night'ın karakteristik yıldız çizimi); gece derinleştikçe sayı ve
  parlaklık artıyor.
- **Solda koyu bir servi silüeti** — yukarı doğru alev gibi uzayan,
  girdap alanını takip eden kısa vuruşlardan oluşan bir motif; tablonun
  imzası.
- **Tuval dokusu:** ince bir SVG `feTurbulence` gren katmanı `overlay`
  blend mode ile üstte durur — ekran değil gerçek bir tuval hissi verir.
- Teknik not: renk karıştırma fonksiyonu hem `#rrggbb` hem `rgb(r,g,b)`
  girişini doğru ayrıştıracak şekilde birleştirildi (önceki sürümde
  hex harfleri regex'i bozup hatalı renklere yol açıyordu).

## 9. Referans Netleşti: Buğday Tarlası Paleti

Ürün sahibi somut bir referans verdi (Van Gogh, "Buğday Tarlasında Yol"
tarzı): koyu/gece mistik ton değil, **gündüz, sıcak, doygun, kalın
darbeli** bir kompozisyon. Buna göre manzara üçüncü kez revize edildi:

- **Gökyüzü paleti** canlı Provence mavisine çekildi (öğlen tonları
  artık `#0e6fc9`/`#2f97dd` gibi doygun mavi-camgöbeği), fırça
  darbeleri kalınlaştırıldı (4–10px, öncekinin ~2 katı) ve sayısı
  azaltılıp okunaklılık artırıldı — az ve güçlü darbe, çok ve gürültülü
  değil. Gündüz paletine beyaz/krem bulut vuruşları eklendi.
- **Servi yerine yuvarlak yapraklı bir ağaç:** solda, kısa gövde
  üzerinde yüzlerce küçük oval yaprak lekesinden oluşan dolgun bir
  taç — referans tablodaki ağacın yorumu. Gece koyulaştıkça yaprak
  paleti koyu yeşile/siyaha kayıyor.
- **Dağ silüeti yerine buğday tarlası:** ön planda çim/çayır şeridi ve
  altında, taranmış gibi tutarlı çapraz yönde binlerce altın/hardal
  rengi vuruştan oluşan bir tarla — Van Gogh'un karakteristik "taranmış"
  buğday dokusu. Zemin, gece ilerledikçe altın tondan koyu çiviteye
  kayan ayrı bir alt-boyama üzerine oturuyor (önceki sürümde ufuk rengi
  kullanılıyordu, gün ortasında neredeyse beyaza çıkıp tarlayı soluk
  gösteriyordu — düzeltildi).
- İç Hava Durumu mekaniği aynı kaldı (Clear/Radiant/Heavy/Stormy fırça
  davranışını ve paleti değiştiriyor); yalnızca "varsayılan" görsel dil
  artık bu referansa sadık.

## 10. Dolu Tuval: Tam Sahne

Dördüncü revizyon ("daha dolu dolu olsun" + üç ek referans: Arles
yakınında ağaçlı yol, Les Alyscamps, Provence'ta çiftlik evi): tuvalin
her santimi boyalı, eksiksiz bir kompozisyon.

- **Gökyüzü tam kaplama:** jitter'lı ızgara geçişi + üstüne serbest
  vurgu geçişi — düz gradyan artık hiçbir yerde görünmüyor, gök baştan
  başa çırpıntılı impasto.
- **Sarı çiftlik evi:** turuncu çatılı, mavi-yeşil kapı/pencereli iki
  bloklu ev ufka yerleşti; **gece olunca bir penceresi sıcak sarı
  yanıyor** — panelin en insani detayı.
- **Çayır bandı:** ufuk çizgisinde koyu çit-çalı sırası, altında parlak
  yeşil çayır vuruşları ve beyaz çiçek + kırmızı gelincik lekeleri
  (referanstaki çiçekli şerit).
- **Buğday tarlası yoğunlaştı:** ızgara garantili tam kaplama, altın
  tonların arasına mavi-gri saplar ve yeşil tutamlar serpiştirildi —
  birebir referansın dokusu.
- **Patika:** sol alttan eve doğru kıvrılan krem/açık sarı taranmış
  vuruşlardan yol ("Via"nın kendisi — tablodaki yol, uygulamanın
  metaforunu taşıyor).
- **Ağaç büyüdü ve koyulaştı:** çift gövde (arkada ince ikinci gövde —
  ağaçlı yol hissi), 500+ yaprak lekesi, koyu yeşil kütlenin içinde
  sarı/kızıl kıvılcım lekeler; gövdelerde referanstaki mavi ton.
- Gece tüm sahne çivite gömülür, yıldız haleleri ve evin penceresi
  kalır — gündüz Arles, gece Yıldızlı Gece.

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
