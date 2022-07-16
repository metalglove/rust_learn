import './index.css';

const rust = import('basics-webgl');
const canvas = document.getElementById('canvas');
const gl = canvas.getContext('webgl2');

rust.then(rustModule => {
  if (!gl) {
    alert('Failed to initialize WebGL2');
    return;
  }

  const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
  var lastDrawTime = -1; // in milliseconds
  const client = new rustModule.Client();
  const initialTime = Date.now();

  function render() {
    window.requestAnimationFrame(render);
    const currentTime = Date.now();

    if (currentTime >= lastDrawTime + FPS_THROTTLE) {
      lastDrawTime = currentTime;

      if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
        canvas.height = window.innerHeight * 0.9;
        canvas.style.height = window.innerHeight * 0.9;

        canvas.width = window.innerWidth * 0.9;
        canvas.style.width = window.innerWidth * 0.9;

        gl.viewport(0, 0, window.innerWidth * 0.9, window.innerHeight * 0.9);
      }

      let elapsedTime = currentTime - initialTime;
      client.update(elapsedTime, window.innerHeight * 0.9, window.innerWidth * 0.9);
      client.render();
    }
  }

  render();
}).catch((error) => console.error(error));