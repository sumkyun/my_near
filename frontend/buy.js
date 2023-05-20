import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;
//const CONTRACT_ADDRESS = "dev-1684600008709-31870777384001";

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

// Setup on page load
window.onload = async () => {
    let isSignedIn = await wallet.startUp();

    if (isSignedIn) {
        signedInFlow();
    } else {
        signedOutFlow();
    }

    console.log(wallet);
    console.log("window loaded");
    getItem();
}

async function getItem() {

    const itemNum = await wallet.viewMethod({
        method: 'get_number_of_items',
        contractId: CONTRACT_ADDRESS
    });

    console.log(itemNum);

    const itemListElement = document.getElementById("item-list");
    itemListElement.innerHTML = "";

    for (let i = 0; i < itemNum; i++) {
        const item = await wallet.viewMethod({
            method: 'get_item',
            args: { item_id: i },
            contractId: CONTRACT_ADDRESS
        });

        const itemContainer = document.createElement("div");
        itemContainer.classList.add("col-xl-3", "col-lg-4", "col-md-5", "col-sm-6");

        const createBox = document.createElement("div");
        createBox.classList.add("create-box");
        itemContainer.appendChild(createBox);

        const link = document.createElement("a");
        link.onclick = function () {
            const idx = i;
            window.location.href = `detail.html?idx=${idx}`;
        };
        createBox.appendChild(link);

        const image = document.createElement("img");
        image.src = item.src;
        image.alt = "";
        link.appendChild(image);

        const title = document.createElement("h3");
        title.textContent = item.title;
        link.appendChild(title);

        const price = document.createElement("p");
        price.style.color = "lightskyblue";
        price.style.marginTop = "20px";
        price.style.fontSize = "20px";
        price.textContent = `${item.price} Near`;
        link.appendChild(price);

        link.appendChild(image);


        itemListElement.appendChild(itemContainer);
    }
}

// UI: Display the signed-out-flow container
function signedOutFlow() {
    // document.querySelector('#signed-in-flow').style.display = 'none';
    // document.querySelector('#signed-out-flow').style.display = 'block';
}

// UI: Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
    // document.querySelector('#signed-out-flow').style.display = 'none';
    // document.querySelector('#signed-in-flow').style.display = 'block';
    // document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    //   el.innerText = wallet.accountId;
    // });
}