import 'regenerator-runtime/runtime';
import { Wallet } from '../../near-wallet';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;
//const CONTRACT_ADDRESS = "dev-1684582956578-27867779489882";

console.log(process.env.CONTRACT_NAME, 'process.env.CONTRACT_NAME')

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

    console.log("window loaded");
    console.log(wallet);
    const params = new URLSearchParams(window.location.search);
    const idx = params.get('idx');
    getItem2(idx);
}

async function getItem2(i) {
    i = parseInt(i);
    const item = await wallet.viewMethod({
        method: 'get_item',
        args: { item_id: i },
        contractId: CONTRACT_ADDRESS
    });

    const itemListElement = document.getElementById("item");
    const itemDetailsHTML = `
        <div class="item-details ov-hidden">
            <h2 class="product-title">${item.title}</h2>
            <div class="favorites">
                <div class="love-react-wrap d-flex align-items-center">
                    <div class="love-react style--two"></div>
                    <div class="love-count">13.6k</div>
                </div>
            </div>
            <p>${item.description}</p>
            <div class="row pt-1">
                <div class="col-sm-6">
                    <div class="price mb-4 mb-sm-0">
                        <h6>Item Price</h6>
                        <h3>${item.price} Near</h3>
                    </div>
                </div>
                <div class="col-sm-6">
                    
                </div>
            </div>
            <div class="row mb-4 mt-30 pt-2">
                <div class="col-sm-6">
                    
                </div>
                <div class="col-sm-6">
                    
                </div>
            </div>
            <div class="row mb-4 mt-40 pt-1">
                <div class="col-sm-6">
                    <a href="" class="hz-profile creator media mb-4 mb-sm-0">
                        <div class="avatar">
                            <img src="${item.src}" alt="">
                        </div>
                        <div class="content media-body">
                            <h6>Seller</h6>
                            <h5>${item.seller}</h5>
                        </div>
                    </a>
                </div>
                <div class="col-sm-6">
                    
                </div>
            </div>
            <div class="tab-content">
                <div class="tab-pane fade show active" id="info">
                    
                </div>
                <div class="tab-pane fade" id="bids">
                    <p>No active bids yet. Be the first to make a bid!</p>
                </div>
            </div>
        </div>
    `;

    itemListElement.innerHTML = itemDetailsHTML;
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