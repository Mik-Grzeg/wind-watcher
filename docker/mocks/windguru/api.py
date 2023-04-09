import falcon, json, logging


class CookiesSetter(object):
    cookies = dict(session='1687f46c2ea20bf45f5b1aeea8209541',
    deviceid='9a9936815593629d897e4dfb8fb25058',
    langc='en-')

    def on_get(self, req, resp, spot_id):
        if spot_id == 36048:
            for cookie_name, cookie_val in self.cookies.items():
                resp.set_cookie(cookie_name, cookie_val, max_age=3000, domain='www.windguru.cz') 
        else:
            resp.status = falcon.HTTP_404

class ForecastResource:
    def __init__(self):
        with open('meta_spot.json', 'r') as f:
            self.response_body_meta = json.load(f)

        with open('forecast_spot.json', 'r') as f:
            self.response_body_forecast = json.load(f)

    def on_get(self, req, resp):
        for cookie_name, cookie_val in CookiesSetter.cookies.items():
            if req.get_cookie_values(cookie_name)[0] != cookie_val or req.get_header('Referer') != 'https://www.windguru.cz':
                resp.status = falcon.HTTP_403
                return
            
        resp.content_type = falcon.MEDIA_JSON

        method = req.params.get('q')

        if method == 'forecast_spot':
            if req.params.get('id_spot') == '36048':
                 resp.text = json.dumps(self.response_body_meta)
            else: 
                return falcon.HTTPNotFound
        elif method == 'forecast':
            if req.params.get('id_model') == '3' \
                    and req.params.get('rundef') == '2023040812x0x240x0x240-2023040800x243x384x255x384' \
                    and req.params.get('initstr') == '2023040812' \
                    and req.params.get('id_spot') == '36048' \
                    and req.params.get('WGCACHEABLE') == '21600' \
                    and req.params.get('cachefix') == '27.85x-15.35x0':
                resp.text = json.dumps(self.response_body_forecast)
        else: 
            resp.status = falcon.HTTP_400


api = falcon.App()
api.add_route('/{spot_id:int}', CookiesSetter())
api.add_route('/int/iapi.php', ForecastResource())

