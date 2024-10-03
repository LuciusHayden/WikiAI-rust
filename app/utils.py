import requests
from bs4 import BeautifulSoup
from openai import OpenAI
from dotenv import load_dotenv
import os 
from .config import OPENAI_KEY, ORGANIZATION_KEY


client = OpenAI(
    organization=ORGANIZATION_KEY, 
    api_key=OPENAI_KEY,
)

def check_link(link):
    try:
        response = requests.get(link)
        if response.status_code == 200:
            return link
        else:
            return redirect_to_wayback(link)
    except:
        return redirect_to_wayback(link)
    
def redirect_to_wayback(url):
    wayback_url = f"https://web.archive.org/web/*/{url}"
    return wayback_url


def process_data(url, question):
    url = url
    response = requests.get(url)
    html = response.text
    soup = BeautifulSoup(html, 'html.parser')
    context = []

    references = soup.find_all('ol', {'class' :'references'})

    for reference in references:
        links = reference.find_all('a', href=True)

    for link in links:
        if 'http' not in link['href']:
            continue
        print(link['href'])
        count+=1
        context.append(link['href'])

    print(count)
    messages = [{"role": "system", "content": content} for content in context]
    messages.append({"role": "system", 
                    "content": "Use only the previous links to answer any questions that you are given, if the links dont contain the answer then state 'No answer found', after answering the question, supply the link that you used to get the answer" 
    })

    messages.append({"role": "user", "content": question})

    response = client.chat.completions.create(
        messages=messages,
        model="gpt-3.5-turbo",
        temperature=0.8,
        max_tokens=100
    )

    source_line = response.choices[0].message.content.split("\n")[-1]  # Get the last line
    response_source = source_line.split(": ")[-1]  # Split on ": " and take the last part
    response.choices[0].message.content + check_link(response_source)

    print(response.choices[0].message.content)
    
    return response.choices[0].message.content

    
