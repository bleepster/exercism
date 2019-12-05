def plxng(number, factor, drop_string):
    return drop_string if number % factor == 0 else ""

def convert(number):
    raindrops = ""
    raindrops += plxng(number, 3, "Pling")
    raindrops += plxng(number, 5, "Plang")
    raindrops += plxng(number, 7, "Plong")
    return str(number) if not raindrops else raindrops
