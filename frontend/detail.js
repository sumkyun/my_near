import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;
//const CONTRACT_ADDRESS = "dev-1684614957495-79136120785832"

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
    console.log(idx);
    getItem2(idx);
}

async function getItem2(i) {
    i = parseInt(i);
    const item = await wallet.viewMethod({
        method: 'get_item',
        args: { item_id: i },
        contractId: CONTRACT_ADDRESS
    });

    console.log(item);

    const itemDetailsElement = document.getElementById('here');
    itemDetailsElement.innerHTML = `
    <!-- Item Details Content -->
                    <div class="item-details ov-hidden">
                        <h2 class="product-title">${item.title}</h2>

                        <!-- Favorite -->
                        <div class="favorites">
                            <div class="love-react-wrap d-flex align-items-center">
                                <div class="love-react style--two"></div>
                                <div class="love-count">13.6k</div>
                            </div>
                        </div>
                        <!-- End Favorite -->

                        <p>${item.description}</p>

                        <div class="row pt-1">
                            <div class="col-sm-6">
                                <!-- Price -->
                                <div class="price mb-4 mb-sm-0">
                                    <h6>Item Price</h6>
                                    <h3>${item.price} Near</h3>
                                </div>
                                <!-- End Price -->
                            </div>
                            <div class="col-sm-6">
                                <!-- Countdown -->
                                
                                <!-- End Countdown -->
                            </div>
                        </div>

                        <div class="row mb-4 mt-30 pt-2">
                            <div class="col-sm-6">

                            </div>
                            <div class="col-sm-6">
                                <!-- Item Price -->
                                
                                <!-- End Item Price -->
                            </div>
                        </div>
                        <img src="${item.src}" alt="" class="svg">
                        <div class="button-group style--two" style="padding: 20px;">
                        <a href="#" class="btn btn-border btn-sm" data-toggle="modal" data-target="#myModal">
                        Buy Now
                    </a>
                    
                        </div>
                        
                        <div class="modal fade" id="myModal" tabindex="-1" role="dialog" aria-labelledby="myModalLabel"
                            aria-hidden="true">
                            <div class="modal-dialog modal-dialog-centered" role="document">
                                <div class="modal-content">
                                    <div class="modal-header">
                                        <h4 style="color:#48006c" class="modal-title" id="myModalLabel">NOTICE</h4>
                                        <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                                            <span aria-hidden="true">&times;</span>
                                        </button>
                                    </div>
                                    <div class="modal-body">
                                        <p style="color:#660099;"> 
                                            Are you going to send ${item.price} NEAR to our safety pool? </p>
                                    </div>
                                    <div class="modal-footer">
                                        <button type="button" class="btn btn-secondary"
                                            data-dismiss="modal">Okay</button>
                                        <button type="button" class="btn btn-primary"  data-dismiss="modal">No</button>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="row mb-4 mt-40 pt-1">
                            <div class="col-sm-6">
                                <!-- Creator -->
                                <a class="hz-profile creator media mb-4 mb-sm-0">
                                    <div class="avatar">
                                        <img src="assets/img/media/creator-avatar.png" alt="">
                                    </div>
                                    <div class="content media-body">
                                        <h6>Seller</h6>
                                        <h5>${item.seller}</h5>
                                    </div>
                                </a>
                                <!-- End Creator -->
                            </div>
                            <div class="col-sm-6"></div>
                        </div><div class="tab-content">
                            <div class="tab-pane fade show active" id="info"></div>
                            <div class="tab-pane fade" id="bids">
                                <p>No active bids yet. Be the first to make a bid!</p>
                            </div>
                        </div>
                    </div>
                    <!-- End Item Details Content -->
                </div>
                <div class="col-xl-5 col-lg-6 order-0 order-lg-1">
                    <!-- Item Details IMG -->
                    <div class="item-details-img mb-5 mb-lg-0">
                        <img src="assets/img/media/item-details.png" alt="">
                    </div>
                    <!-- End Item Details IMG -->
                
    
    `
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