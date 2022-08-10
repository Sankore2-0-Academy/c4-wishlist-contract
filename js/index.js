if (window.location.pathname === '/auth.html') {
  // Check if wallet is linked
  if (isLoggedIn()) {
    window.location.href = '/index.html';
  } else {
    // Access login button DOM element
    const loginBtn = document.querySelector('#login-btn');
    loginBtn.onclick = (event) => {
      login();
    };
  }
} else if (
  window.location.pathname === '/new.html'
) {
  // Check if logged in
  if (!isLoggedIn()) {
    window.location.href = '/auth.html';
  }

  const newCarForm = document.querySelector('#new-car');
  // Adds submitted task
  newCarForm.onsubmit = async (event) => {
    event.preventDefault();
    const image = event.target.image.value;
    const name = event.target.name.value;
    const model = event.target.model.value;
    const mileage = event.target.mileage.value;
    const year = event.target.year.value;
    const price = event.target.price.value;
    
    const addedCar = await addCar(
      image,
      name,
      model,
      Number.parseInt(mileage),
      year,
      Number.parseFloat(price)
    );

    if (addedCar != null) {
      window.location.href = '/index.html';
    }
    newCarForm.reset();
  };

} else if (
  window.location.pathname === '/index.html' ||
  window.location.pathname === '/'
) {
  if (!isLoggedIn()) {
    window.location.href = '/auth.html';
  }
  document.querySelector('#wallet-id').textContent = getAccount();

  const logoutBtn = document.querySelector('#logout');

  /**
   * =====================================================
   * Smart Contract Calls
   * =====================================================
   */

  // Logout User
  logoutBtn.onclick = () => {
    logout();
    window.location.reload();
  };


  // Displays Wishlist
  displayWishlist();
}

async function displayWishlist() {
  // Retreive cars records in wishlist
  const wishlist = await readWishlist(0, 50);
  let carRecords = '';

  // Loop through the list of tasks if any and wrap them in relevant HTML elemnts
  for (let i = 0; i < wishlist.length; i++) {
    const car = wishlist[i];
    carRecords += carElement(i, car);
  }

  // Reflect the change in DOM
  updateCarRecords(carRecords);
}

function carElement(id, {image, name, model, mileage, year, price}) {
  return `<tr class="fw-normal">
                      <th>
                        <img
                          src="${image}"
                          alt="Toyota"
                          style="width: 45px; height: auto"
                        />
                        <span class="ms-2">${name}</span>
                      </th>
                      <td class="align-middle">
                        <span>${model}</span>
                      </td>
                      <td class="align-middle">
                        <span>${mileage}</span>
                      </td>
                      <td class="align-middle">
                        <span>${year}</span>
                      </td>
                      <td class="align-middle">
                        <span>$${price}</span>
                      </td>
                      <td class="align-middle">
                        <a href="#!" onclick="deleteCar(${id})" data-mdb-toggle="tooltip" title="Remove"
                          ><i class="fa fa-trash fa-lg text-danger"></i
                        ></a>
                      </td>
                    </tr>`;
}

function updateCarRecords(carRecords) {
  const carRecordsView = document.querySelector('#car-records');
  carRecordsView.innerHTML = carRecords;
}

// async function removeCar(id) {
//   const res = await delete_car(id);
//   window.location.reload();
// }
