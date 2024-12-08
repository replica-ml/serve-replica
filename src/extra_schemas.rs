#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub(crate) struct ScraperPostBody {
    url: String,
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct ScraperPostBodyResponse {
    pub success: bool,
    pub id: String,
    pub url: String,
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CrawledResult {
    pub completed: i64,
    pub credits_used: i64,
    pub data: Vec<Daum>,
    pub expires_at: String,
    pub status: String,
    pub success: bool,
    pub total: i64,
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Daum {
    pub markdown: String,
    pub metadata: Metadata,
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Metadata {
    pub description: String,
    pub keywords: String,
    #[serde(rename = "og:image")]
    pub og_image: String,
    #[serde(rename = "og:title")]
    pub og_title: String,
    #[serde(rename = "og:url")]
    pub og_url: String,
    #[serde(rename = "ogImage")]
    pub og_image2: String,
    pub og_locale_alternate: Vec<serde_json::Value>,
    #[serde(rename = "ogTitle")]
    pub og_title2: String,
    #[serde(rename = "ogUrl")]
    pub og_url2: String,
    #[serde(rename = "p:domain_verify")]
    pub p_domain_verify: String,
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    pub status_code: i64,
    pub title: String,
    pub url: String,
    pub viewport: String,
}

lazy_static::lazy_static! {
    pub(crate) static ref EXAMPLE_SCRAPED_RESULT: serde_json::Value = serde_json::json!({
        "completed": 1,
        "creditsUsed": 1,
        "data": [
            {
                "markdown": "1.  [Suits & Tuxedos](/suits-tuxedos/)\n    \n2.  [Navy Stretch Shawl Lapel Tuxedo Separates](#)\n    \n\n![Friar Tux](/on/demandware.static/Sites-FriarTux-Site/-/default/dwf271f94d/images/Stitch-and-Tie-Logo-Black.png)\n\nBuild Your Look\n\nBack to Products ![Close](/on/demandware.static/Sites-FriarTux-Site/-/default/dw1cfc9bee/images/close-icon.svg) \n\n*   ![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dwa6d8297d/images/large/friartux-navy-blue-tuxedo-c5450-large-1.png?sw=80&sh=150&sm=fit)\n*   ![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dw5c7091dd/images/large/friartux-navy-blue-tuxedo-c5450-large-2.png?sw=80&sh=150&sm=fit)\n*   ![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dwa45df582/images/large/friartux-navy-blue-tuxedo-c5450-large-3.png?sw=80&sh=150&sm=fit)\n*   ![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dw7946cc4b/images/large/friartux-navy-blue-tuxedo-c5450-large-4.png?sw=80&sh=150&sm=fit)\n\n![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dwa6d8297d/images/large/friartux-navy-blue-tuxedo-c5450-large-1.png?sw=752&sh=1252&sm=fit)\n\n![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dw5c7091dd/images/large/friartux-navy-blue-tuxedo-c5450-large-2.png?sw=752&sh=1252&sm=fit)\n\n![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dwa45df582/images/large/friartux-navy-blue-tuxedo-c5450-large-3.png?sw=752&sh=1252&sm=fit)\n\n![Navy Stretch Shawl Lapel Tuxedo Separates image number null](https://www.friartux.com/dw/image/v2/BFTS_PRD/on/demandware.static/-/Sites-friartux-catalog-m/default/dw7946cc4b/images/large/friartux-navy-blue-tuxedo-c5450-large-4.png?sw=752&sh=1252&sm=fit)\n\nShare this look\n\n*   [](http://www.pinterest.com/pin/create/button/?url=https%3A%2F%2Fwww.friartux.com%2Fsuits-tuxedos%2Fnavy-stretch-shawl-lapel-tuxedo-separates%2FFT-C5450.html&media=https%3A%2F%2Fwww.friartux.com%2Fdw%2Fimage%2Fv2%2FBFTS_PRD%2Fon%2Fdemandware.static%2F-%2FSites-friartux-catalog-m%2Fdefault%2Fdwa6d8297d%2Fimages%2Flarge%2Ffriartux-navy-blue-tuxedo-c5450-large-1.png%3Fsw%3D752%26sh%3D1252%26sm%3Dfit&description=Navy Stretch Shawl Lapel Tuxedo Separates \"Create a Pinterest Pin for Navy Stretch Shawl Lapel Tuxedo Separates\")\n    \n*   [](https://www.facebook.com/share.php?u=https%3A%2F%2Fwww.friartux.com%2Fsuits-tuxedos%2Fnavy-stretch-shawl-lapel-tuxedo-separates%2FFT-C5450.html \"Share Navy Stretch Shawl Lapel Tuxedo Separates on Facebook\")\n    \n\nYour Additional Items\n\n1.  [Suits & Tuxedos](/suits-tuxedos/)\n    \n2.  [Navy Stretch Shawl Lapel Tuxedo Separates](#)\n    \n\nNavy Stretch Shawl Lapel Tuxedo Separates\n=========================================\n\n~Price reduced from $190.00 to~ $133.00\n\nItem No. FT-C5450-40R\n\n4.3 out of 5 Customer Rating 25 REVIEWS\n\nBlue\n\nAvailability:\n\n*   Select Styles for Availability\n    \n\n*    Add & Customize Look\n\nBuy from\n\n$259.95 $181.96\n\nUsually ships in 2-3 business days\n\n[Find Your Fit\\\n\\\nEasily find your size by answering a few simple questions](https://www.friartux.com/fitfinder?returnPid=FT-C5450)\n\nSelect a size 34S 36S 36R 38S 38R 38L 40S 40R 40L 42S 42R 42L 44S 44R 44L 46S 46R 46L 48S 48R 48L 50S 50R 50L 52S 52R 52L 54R 54L 56R 58R\n\nQuantity 1 2 3 4 5 6 7 8 9 10\n\nIf you are only building a look, you do not need to select a size\n\nAdd & Customize Look\n\nRent & Customize Look\n\n### Product Details\n\nfit\n\nSlim\n\nSizes Available\n\n34S-48S; 36R-56R; 38L-54L\n\nLapel\n\nshawl\n\nFabric\n\nPoly-Viscose-Spandex\n\nBrand\n\nCouture 1910\n\nAh a navy tuxedo with a black shawl lapel, is there anything cooler than that? (Not in our opinion) We love the option of adding black pants or navy pants to this navy tuxedo with shawl lapel, plus the option of a black or white shirt. Finish off your look with a black tie and black shoes and you're ready for the red carpet, casino or wedding ceremony!\n\nStretch\n\n*   [description](#description)\n    \n*   [additional information](#addInfo)\n    \n*   [reviews](#reviews)\n    \n\nAh a navy tuxedo with a black shawl lapel, is there anything cooler than that? (Not in our opinion) We love the option of adding black pants or navy pants to this navy tuxedo with shawl lapel, plus the option of a black or white shirt. Finish off your look with a black tie and black shoes and you're ready for the red carpet, casino or wedding ceremony!\n\n|     |     |\n| --- | --- |\n| Product # | Lorem ipsum dolor sit amet |\n| Available packaging | LOLDuis aute irure dolor in reprehenderit |\n| Weight | dolor sit amet |\n| Sunt in culpa qui | Lorem ipsum dolor sit amet |\n\n|     |     |\n| --- | --- |\n| Weight | dolor sit amet |\n| Sunt in culpa qui | Lorem ipsum dolor sit amet |\n| Product # | Lorem ipsum dolor sit amet |\n| Available packaging | LOLDuis aute irure dolor in reprehenderit |\n\n![Han Solo](/on/demandware.static/Sites-FriarTux-Site/-/default/dw56896117/images/reviewavatar.png)Dec 2018\n\n##### Han Solo\n\nOne morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections\n\n![Luke Skywalker](/on/demandware.static/Sites-FriarTux-Site/-/default/dw56896117/images/reviewavatar.png)Dec 2018\n\n##### Luke Skywalker\n\nThe bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream.\n\n##### Leave a review\n\nYour name \\* \n\nYour rating \\* ★★★★★ (5/5) ★★★★☆ (4/5) ★★★☆☆ (3/5) ★★☆☆☆ (2/5) ★☆☆☆☆ (1/5)\n\nYour email \\* \n\nReview text \\*\n\nPost review",
                "metadata": {
                    "description": "Navy Stretch Shawl Lapel Tuxedo Separates from Friar Tux.",
                    "keywords": "Friar Tux",
                    "og:image": "https://www.friartux.com/on/demandware.static/-/Sites-friartux-catalog-m/default/dwa6d8297d/images/large/friartux-navy-blue-tuxedo-c5450-large-1.png",
                    "og:title": "Navy Stretch Shawl Lapel Tuxedo Separates | FriarTux",
                    "og:url": "https://www.friartux.com/suits-tuxedos/navy-stretch-shawl-lapel-tuxedo-separates/FT-C5450.html",
                    "ogImage": "https://www.friartux.com/on/demandware.static/-/Sites-friartux-catalog-m/default/dwa6d8297d/images/large/friartux-navy-blue-tuxedo-c5450-large-1.png",
                    "ogLocaleAlternate": [],
                    "ogTitle": "Navy Stretch Shawl Lapel Tuxedo Separates | FriarTux",
                    "ogUrl": "https://www.friartux.com/suits-tuxedos/navy-stretch-shawl-lapel-tuxedo-separates/FT-C5450.html",
                    "p:domain_verify": "c0442844bff66429c20ec2a1df7160d0",
                    "sourceURL": "https://www.friartux.com/suits-tuxedos/navy-stretch-shawl-lapel-tuxedo-separates/FT-C5450.html",
                    "statusCode": 200,
                    "title": "Navy Stretch Shawl Lapel Tuxedo Separates | Friar Tux",
                    "url": "https://www.friartux.com/suits-tuxedos/navy-stretch-shawl-lapel-tuxedo-separates/FT-C5450.html",
                    "viewport": "width=device-width, initial-scale=1"
                }
            }
        ],
        "expiresAt": "2024-12-04T02:54:14.000Z",
        "status": "completed",
        "success": true,
        "total": 1
    });
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub struct SwapPostRequest {
    pub user_img_url: String,
    pub model_img_url: String,
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub struct SwapPostResponse {
    pub output_url: String,
    pub status: String,
}
