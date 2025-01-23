import { Component, OnInit } from '@angular/core';
import { AiComponent, AiResponse, Reference, References } from '../../../interfaces/wiki-query';
import { AiInputService } from '../../../services/ai-input.service';
import { FormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-ai-input',
  standalone: true,
  imports: [FormsModule, HttpClientModule, CommonModule],
  templateUrl: './ai-input.component.html',
  styleUrl: './ai-input.component.css'
})
export class AiInputComponent {

  component : AiComponent = {
      input : { query :  "" },
      response : { response: "" },
      references : { references : []},
      main_reference : { link : "", id : 0},
  }

  main_reference_change : Reference = {
    link : "",
    id : 0, // this vaue doesnt matter, the backend doesnt use it
  }

  constructor(private _aiInputService: AiInputService) {
  }

  ngOnInit() {
    this.reload_references();
  }

  reload_references() : void {
    this._aiInputService.getMainReference().subscribe((reference : Reference) => {
      this.component.main_reference = reference;
    });
     this._aiInputService.getReferences().subscribe((references : References) => {
      this.component.references = references;
    });
  }

  processWikiSite() : void {
      this._aiInputService.processWikiSite(this.component.input).subscribe((response : AiResponse) => {
        this.component.response = response;
      })
  }

  setReferences() : void {
    this._aiInputService.setReferences(this.main_reference_change).subscribe();
    this.reload_references();
  }
}
