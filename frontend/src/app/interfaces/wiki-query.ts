export interface AiResponse {
  response : string
}

export interface AiInput {
  query : string
}

export interface Reference {
  link : String
}

export interface References {
  references : Reference[]
}




export interface AiComponent {
  response :  AiResponse,
  input : AiInput,
  references : References,
  main_reference : Reference
}
