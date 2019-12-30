import("../pkg/index.js")
  .then(({ Mazm }) => {
    const pre = document.getElementById("mazm");
    const input = document.getElementById("img");
    const sz = 55;

    let mazm;
    function renderer() {
      pre.innerHTML = mazm.render();
      mazm.tick(16);

      requestAnimationFrame(renderer);
    }

    input.addEventListener("change", ({ target }) => {
      if (target.files && target.files[0]) {
        const reader = new FileReader();
        reader.addEventListener("load", () => {
          const img = new Image();
          img.addEventListener("load", () => {
            const c = document.createElement("canvas");
            c.width = sz;
            c.height = sz;

            const ctx = c.getContext("2d");
            ctx.drawImage(img, 0, 0, sz, sz);
            const dat = ctx.getImageData(0, 0, sz, sz).data;

            mazm = Mazm.new(dat, sz);
            requestAnimationFrame(renderer);
          });

          img.width = sz;
          img.height = sz;
          img.src = reader.result;
        });

        reader.readAsDataURL(target.files[0]);
      }
    });
  })
  .catch(console.error);
