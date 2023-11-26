import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { TokenService } from '../../service/token.service';
import { TokenInfo } from '../../models/token-info';
import { LoggingService } from '../../service/logging.service';
import { MessageService } from 'primeng/api';
import { Subscription, interval, startWith } from 'rxjs';

@Component({
  selector: 'app-token-list',
  templateUrl: './token-list.component.html',
  styleUrls: ['./token-list.component.scss']
})
export class TokenListComponent implements OnInit {
  tokens: TokenInfo[] = [];
  loading: boolean = false;
  private tokensSubscription: Subscription;
  private tokenUpdateSubscription: Subscription;
  private updateIntervalSubscription: Subscription;

  constructor(private tokenService: TokenService, private logging: LoggingService
    ,private messageService: MessageService, private cdr: ChangeDetectorRef) {}

  async ngOnInit(): Promise<void> {
    this.loading = true;
    try {
      this.logging.info('Loading tokens...');

      this.tokensSubscription = this.tokenService.tokens$.subscribe(
        data => {

            this.tokens = data;
            this.normalizeTokens();
            this.updateTokens();
            this.loading = false;
        }
      );
      this.loading = true;
      this.tokenService.fetchTokens(); // Initial token fetch
      //this.tokens = tokenData;

      this.updateIntervalSubscription = interval(60000) // 60000 ms = 1 minute
      .pipe(startWith(0)) // Start immediately
      .subscribe(() => this.updateTokens());

    } catch (error) {
      this.logging.error('Error loading tokens:', error);
    }


  }
  normalizeTokens() {
    this.logging.info('Normalizing tokens...');


    for (let token of this.tokens) {
      // Existing logic to form fullName
      let name = token.name || '';
      let symbol = token.symbol || '';
      let fullName = name;
      if (name.length > 40) {
        fullName = name.substring(0, 50);
      }
      if (symbol.length > 0) {
        fullName += ' (' + symbol + ')';
      }

      // Parse the UTC dateCreated and calculate minutes since creation

      // Append minutes ago to fullName
   //  fullName += ` - ${timeAgo}`;

      token['fullName'] = fullName;
    }
  }

  updateTokens(){
    this.logging.info('Updating tokens...');
    const currentTime = new Date();
    for(let token of this.tokens){
        const createdDate = new Date(token.dateCreated + 'Z'); // Add 'Z' to indicate UTC time
        const diffInMinutes = Math.round((currentTime.getTime() - createdDate.getTime()) / 60000); // 60000 milliseconds in a minute
        let timeAgo = 'Just now';
        if(diffInMinutes > 1440){
            timeAgo = `${Math.round(diffInMinutes / 1440)} days ago`;
        }
        else
        if(diffInMinutes > 60){
            timeAgo = `${Math.round(diffInMinutes / 60)} hours ago`;
        }
        else if(diffInMinutes > 0){
            timeAgo = `${diffInMinutes} minutes ago`;
        }
        token.createdString = timeAgo;
    }
  }


  copyAddress(address: string): void {
    navigator.clipboard.writeText(address).then(() => {
      this.messageService.add({severity:'success', summary: 'Success', detail: 'Address copied to clipboard!'});
    }, (err) => {
      this.messageService.add({severity:'error', summary: 'Error', detail: 'Failed to copy address'});
    });
  }
  goToEtherscan(address: string): void {
    const url = `https://etherscan.io/token/${address}`;
    window.open(url, "_blank");
  }
  ngOnDestroy(): void {
    if (this.tokensSubscription) {
      this.tokensSubscription.unsubscribe();
    }
  }
}
