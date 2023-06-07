
import { Wallet } from './near-wallet';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

// Setup on page load
window.onload = async () => {
  let isSignedIn = await wallet.startUp();

//   if (isSignedIn) {
//     signedInFlow();
//   } else {
//     signedOutFlow();
//   }
  console.log(wallet.accountId);
};


console.log(wallet.accountId);

const button = document.getElementById('upload-btn');


async function sendData(){


    var title = document.getElementById("title").value;
    var price = document.getElementById("price").value;
    var description = document.getElementById("description").value;

    var jsonData = {
    title: title,
    description: description,
    src:"asset/img/ad.png",
    price: price,
    };
    alert("hinnnn");
    console.log("전송할 데이터:", jsonData);
    await wallet.callMethod({method:'add_item', args:{title:title, description:description, src:"asset/img/ad.png", price:price}, contractId:CONTRACT_ADDRESS});
    console.log("return:", ret);
    alert("hihihihihi");
}

button.onclick=sendData;