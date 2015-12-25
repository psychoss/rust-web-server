function collectInfos() {
  var passowrd = inputPassword.value;
  if (inputEmail.validity.valid && passowrd.trim()) {
    return {
      email: inputEmail.value,
      pwd: passowrd
    }
  }
}

function bindSignin(argument) {
  if (inputSubmit) {
    $(inputSubmit).on('click', signin);
  }

}

function signin() {
  console.log(collectInfos()==undefined);

}

$(document).ready(function() {
  bindSignin();
})
