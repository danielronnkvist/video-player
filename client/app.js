let video = document.createElement("video");
video.setAttribute("playsinline", true);
let canvas = document.querySelector("canvas");
let context = canvas.getContext("2d");
// store animation frame
let raf;

function loop(){
 context.drawImage(video, 0, 0, canvas.width, canvas.height);
 raf = requestAnimationFrame(loop);
}

navigator.mediaDevices.getUserMedia({video: true, audio: false}).then((localStream) => {
  video.srcObject = localStream;
  video.onplaying = () => {
    raf = requestAnimationFrame(loop);
  };
  video.play();
  localStream.getTracks().forEach(track => {
    console.log(track, localStream);
  });
}).catch(e => alert(e));

