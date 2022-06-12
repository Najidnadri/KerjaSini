<template>
    <div class="background">
        <div class="container">
            <section class="column">
                <div class="left-column">
                    <h2>Welcome!</h2>
                    <button>
                        LOOKING FOR A JOB? LOGIN HERE
                    </button>
                    <button>
                        LOGIN AS EMPLOYER
                    </button>
                </div>
            </section>
            <section class="column">
                <div class="right-column">
                    <h2>HARD WORK BEATS TALENT WHEN TALENT DOESN'T WORK HARD</h2>
                    <p>TIM NOTKE</p>
                </div>
            </section>
        </div>
    </div>
</template>

<style scoped>


.background {
    position:absolute;
    top:0px;
    right:0px;
    bottom:0px;
    left:0px;
    background-image: linear-gradient(20deg, rgb(222, 115, 188), rgb(65, 0, 65));
    display: flex;
    justify-content: center;
    align-items: center;
}

.container {
    display: grid;
    grid-template-columns: 60% 40%;
    place-items: center;
    background-color: rgb(217, 216, 216);
    margin-top: 7%;
    margin-bottom:  7%;
    margin-right: 20%;
    margin-left: 20%;
    min-height: 80%;
    border-radius: 10px;
}

.column {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50%;
    width: 50%;
    text-align: center;
}

.left-column {
    color: rgb(72, 72, 72);
    display: grid;
    grid-template-rows: 50% 50%;
    font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
}

.right-column {
    color: rgb(72, 72, 72);
    display: grid;
    grid-template-rows: 50% 50%;
    font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
}

button {
  appearance: button;
  backface-visibility: hidden;
  background-color: #405cf5;
  border-radius: 50px;
  border-width: 0;
  box-shadow: rgba(50, 50, 93, .1) 0 0 0 1px inset,rgba(50, 50, 93, .1) 0 2px 5px 0,rgba(0, 0, 0, .07) 0 1px 1px 0;
  box-sizing: border-box;
  color: #fff;
  cursor: pointer;
  font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
  font-size: 100%;
  height: 44px;
  line-height: 1.15;
  margin: 12px 0 0;
  outline: none;
  overflow: hidden;
  padding: 0 25px;
  position: relative;
  text-align: center;
  text-transform: none;
  transform: translateZ(0);
  transition: all .2s,box-shadow .08s ease-in;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
  width: 100%;
}


</style>