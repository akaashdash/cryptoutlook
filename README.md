# cryptoutlook
CS128H Project

## Group Name: CryptOutlook  
## Group Member Names and NetIDs:  
Akaash Dash (akdash2), Yesui Dovchinsuren (ydovc2), Nathaniel Dyer (nedyer2)

## Project Introduction:  
### Description:  
With increasing acceptance of cryptocurrencies and increasing accessibility, it appears that investment in crypto is likely to increase in coming years.  
However, because cryptocurrencies are not backed by any tangible asset, it can be assumed that those attempting to profit off the market would base their patterns of purchasing/selling on those of others and current news.  
In this project, we seek to model these behaviors and explore if and how we can use this behavior to create accurate predictions of future crypto trends.  
### Goals, objectives and why we chose this project:  
Branch and build off of the skills we learned.  
Familiarize ourselves with Machine Learning/Data analysis, natural language processing (NLP) and other networking analysis processes done in Rust.  
Interested in building a simple ML/data analysis and social media sentiment analysis program to predict cryptocurrency performance.  
## System Overview:  
The prerequisite to this project is finding price/volume data of cryptocurrencies (BTC, ETH, SOL, etc.) and the daily open/close prices along with daily news and social media feeds to pair with the times of the cryptocurrency data. This information will be used to train and test our model. To gather this data, we will find sources/APIs online that provide such information and may or may not use rust crates that consume REST APIs (depending on the source) to store this data for frequent usage.  
Our project then requires a form of sentiment analysis and NLP using provided cargo crates to analyze news articles and media posts.  
We will then need to find supported ML rust crates with appropriate algorithms (LinReg, SVM, etc.).  
A final rust crate will then be used to present our findings and allow us to iteratively improve it as we learn more about how to better predict trends (still unclear how we will do this).  
## Possible Challenges:  
We all lack experience with natural language processing (NLP), so we will need to both learn and implement rather than just implementing.  
Neural networks and machine learning can often require a lot of computational power, which may slow progress or limit the achievable scope of the project.  
Systems like these already exist for stocks, so it is possible that analysis like this is already priced into cryptocurrency prices, meaning that price changes may be made counter to our inputs as a result of others seeing negativity as not as negative as predicted. However, that portion of cryptocurrency investors is likely small enough that it does not outweigh the outcomes that media positivity or negativity would intuitively suggest.  
Although cryptocurrency prices are much more based on public perception than stocks are, there are still many other factors that influence the price, which may interfere with our model. Furthermore, our model may not find correlation strong enough to make predictions with statistically significant accuracy, but this would just be less “cool,” not less functional.  

## References:  
https://arxiv.org/pdf/1607.01958.pdf  
https://cs229.stanford.edu/proj2011/GoelMittal-StockMarketPredictionUsingTwitterSentimentAnalysis.pdf  
https://www.ncbi.nlm.nih.gov/pmc/articles/PMC8053016/  
https://github.com/guillaume-be/rust-bert  
https://crates.io/keywords/nlp  
https://guillaume-be.github.io/about/  
https://aclanthology.org/2020.nlposs-1.4/  
https://scholar.smu.edu/cgi/viewcontent.cgi?article=1039&context=datasciencereview  
