import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';

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

    console.log("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    document.getElementById("aaaaa").onclick = getItems;

}

async function getItems() {
    const items = await wallet.viewMethod({
        method: 'get_user_items',
        args: {
            user_id: wallet.accountId
        },
        contractId: CONTRACT_ADDRESS
    });

    document.getElementById("name1").innerHTML=wallet.accountId;
    console.log(items);
    console.log(wallet.accountId);

    const itemListElement = document.getElementById("items");
    itemListElement.innerHTML = ``;
    for (let a in items) {
        if (items[a].state == 'CALL') {
            itemListElement.innerHTML += `
<div class="col-lg-4 col-md-6 item-trade" data-status="판매승인">
    <!-- Single Product -->
    <div class="single-product mb-30">
        <a href="#">
            <img src="${items[a].src}" alt="">
        </a>
        <!-- Product Content -->
        <div class="product-content">
            <div class="product-top">
                <h5  style="display: flex; justify-content: center; ">${items[a].title}</h5>
                <div class="d-flex justify-content-between">
                    <h6>${items[a].price} NEAR</h6>
                    <div class="button-group style--two">
                        <button class="buy-button text-with-hover-effect btn btn-border btn-sm"
                                style="font-weight: bold; margin: -5px;" data-toggle="modal"
                                data-target="#myModal">
                            Approve
                        </button>
                    </div>
                    <!-- 모달 팝업 -->
                    <div class="modal fade" id="myModal" tabindex="0" role="dialog"
                         aria-labelledby="myModalLabel" aria-hidden="true">
                        <div class="modal-dialog modal-dialog-centered" role="document">
                            <div class="modal-content" style="z-index: 100;">
                                <div class="modal-header">
                                    <h4 style="color:#48006c" class="modal-title"
                                        id="myModalLabel">NOTICE</h4>
                                    <button type="button" class="close" data-dismiss="modal"
                                            aria-label="Close">
                                        <span aria-hidden="true">&times;</span>
                                    </button>
                                </div>
                                <div class="modal-body">
                                    <p style="color:#660099;">For secure transaction, 0.5%
                                        deposit will be applied.</p>
                                </div>
                                <div class="modal-footer">
                                    <button type="button" class="btn btn-secondary"
                                            data-dismiss="modal">Okay
                                    </button>
                                    <button type="button" class="btn btn-primary"
                                            data-dismiss="modal">No
                                    </button>
                                </div>
                            </div>
                        </div>

                    </div>
                </div>

            </div>
            <!-- End Product Content -->
        </div>
        <!-- End Single Product -->
    </div>
</div>
    `;
        }
        else {
            itemListElement.innerHTML += `
            <!--col-lg-4가 하나의 덩어리 입니다!-->
            <div class="col-lg-4 col-md-6 item-trade" data-status="구매요청">
                <!-- Single Product -->
                <div class="single-product mb-30">
                    <a href="#">
                        <img src="${items[a].src}" alt="">
                    </a>
                    <!-- Product Content -->
                    <div class="product-content">
                        <div class="product-top">
                            <h5>${items[a].title}</h5>
                            <div class="d-flex justify-content-between">
                                <h6>${items[a].price} NEAR</h6>
                            </div>
                        </div>

                    </div>
                    <!-- End Product Content -->
                </div>
                <!-- End Single Product -->
            </div>
`
        }
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