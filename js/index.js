import("../pkg/index.js")
  .then(({ Mazm }) => {
    const pre = document.getElementById("mazm");
    const input = document.getElementById("img");
    const w = 2**6;

    let mazm;
    let frame;

    function renderer() {
      pre.innerHTML = mazm.render();
      mazm.tick(2**4);

      frame = requestAnimationFrame(renderer);
    }

    input.addEventListener("change", ({ target }) => {
      if (target.files && target.files[0]) {
        const reader = new FileReader();
        reader.addEventListener("load", () => {
          const img = new Image();

          img.addEventListener("load", () => {
            const r = img.naturalWidth / img.naturalHeight;
            const h = w / r;
            const c = document.createElement("canvas");

            c.width = w;
            c.height = h;
            img.height = h;

            const ctx = c.getContext("2d");
            ctx.drawImage(img, 0, 0, w, h);
            const dat = ctx.getImageData(0, 0, w, h).data;

            if (frame) {
              cancelAnimationFrame(frame);
              mazm.free()
            };
            mazm = Mazm.new(dat, w, h);
            frame = requestAnimationFrame(renderer);
          });

          img.width = w;
          img.src = reader.result;
        });

        reader.readAsDataURL(target.files[0]);
      }
    });
  })
  .catch(console.error);
